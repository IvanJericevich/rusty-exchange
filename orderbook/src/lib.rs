#![feature(binary_heap_retain)]
mod queue;

use futures::StreamExt;
use futures::executor;
use chrono::Utc;
use rabbitmq_stream_client::{Environment, Producer, Dedup};
use rabbitmq_stream_client::types::{ByteCapacity, Message, OffsetSpecification};
use database::orders::Order;
use database::{OrderSide, OrderType};
use database::fills::Fill;
use crate::queue::Queue;
use std::time::Duration;

const QUEUE_CAPACITY: usize = 500;

pub struct OrderBook { // TODO: price and size increment
    id: i32,
    bids: Queue,
    asks: Queue,
    producer: Option<Producer<Dedup>>
}

impl OrderBook {
    pub async fn new(market_id: i32) -> Self {
        let producer = if !cfg!(test) {
            // Establish connection to RabbitMQ
            let environment = Environment::builder()
                .host("localhost")
                .port(5552)
                .build()
                .await
                .unwrap();
            let _ = environment.delete_stream("fills").await; // Delete stream if it exists
            environment // Create stream at producer
                .stream_creator()
                .max_length(ByteCapacity::MB(50))
                .max_age(Duration::new(30, 0))
                .create("fills")
                .await
                .unwrap();
            Some(
                environment
                    .producer()
                    .name("fills")
                    .build("fills")
                    .await
                    .unwrap()
            )
        } else {
            None
        };
        OrderBook {
            id: market_id,
            bids: Queue::new(QUEUE_CAPACITY),
            asks: Queue::new(QUEUE_CAPACITY),
            producer
        }
    }
    
    fn process(&mut self, order: Order) -> bool {
        match order.r#type {
            OrderType::Limit => self.process_limit(order),
            OrderType::Market => self.process_market(order),
        }
    }

    fn process_limit(&mut self, order: Order) -> bool {
        if let Some(contra_order) = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => (&mut self.asks).peek().cloned(),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => (&mut self.bids).peek().cloned()
        } {
            if match order.side {
                OrderSide::Buy | OrderSide::Bid | OrderSide::Long => order.price >= contra_order.price,
                OrderSide::Sell | OrderSide::Ask | OrderSide::Short => order.price <= contra_order.price
            } {
                let mut order = order; // Take the previous value out of scope
                if !self.cross(&mut order, contra_order) {
                    self.process_limit(order)
                } else {
                    true
                }
            } else {
                self.store(order)
            }
        } else {
            self.store(order)
        }
    }
    
    fn process_market(&mut self, order: Order) -> bool {
        if let Some(contra_order) = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => (&mut self.asks).peek().cloned(),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => (&mut self.bids).peek().cloned()
        } {
            let mut order = order;
            if !self.cross(&mut order, contra_order) {
                self.process_market(order);
            }
        }
        true
    }

    fn store(&mut self, order: Order) -> bool {
        match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => (&mut self.bids).insert(order),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => (&mut self.asks).insert(order)
        }
    }

    fn cross(&mut self, order: &mut Order, contra_order: Order) -> bool {
        {
            self.publish_fill(
                contra_order.price.unwrap(),
                f32::min(order.size, contra_order.size),
                contra_order.side,
                contra_order.r#type,
                contra_order.sub_account_id,
                contra_order.id
            );
            self.publish_fill(
                contra_order.price.unwrap(),
                f32::min(order.size, contra_order.size),
                order.side.clone(),
                order.r#type.clone(),
                order.sub_account_id,
                order.id
            );
        }
        let contra_queue = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => &mut self.asks,
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => &mut self.bids
        };
        if order.size < contra_order.size { // Modify the contra order
            contra_queue.modify_tob(contra_order.size - order.size);
            true
        } else if order.size > contra_order.size { // Modify the submitted order
            order.size -= contra_queue.pop().unwrap().size;
            false
        } else {
            contra_queue.pop();
            true
        }
    }

    // fn process_order_cancel(&mut self, order: Order) -> bool {
    //     match order.side {
    //         OrderSide::Buy | OrderSide::Bid | OrderSide::Long => (&mut self.bids).cancel(order.id),
    //         OrderSide::Sell | OrderSide::Ask | OrderSide::Short => (&mut self.asks).cancel(order.id),
    //     }
    // }

    pub fn spread(&mut self) -> Option<(f32, f32)> {
        let bid = self.bids.peek()?.price.unwrap();
        let ask = self.asks.peek()?.price.unwrap();
        Some((bid, ask))
    }

    fn publish_fill(
        &mut self,
        price: f32,
        size: f32,
        side: OrderSide,
        r#type: OrderType,
        sub_account_id: i32,
        order_id: i32
    ) {
        if let Some(producer) = &mut self.producer {
            let fill = Fill {
                price,
                size,
                quote_size: price * size,
                side,
                r#type,
                created_at: Utc::now().naive_utc(),
                sub_account_id,
                market_id: self.id,
                order_id,
            };
            let _ = executor::block_on(
                producer
                    .send_with_confirm(
                        Message::builder()
                            .body(serde_json::to_string(&fill).unwrap()) // TODO: Dont confirm otherwise api will halt
                            .build()
                    )
            );
        }
    }
    
    pub async fn run(&mut self) {
        let mut consumer = Environment::builder()
            .host("localhost")
            .port(5552)
            .build()
            .await
            .unwrap()
            .consumer()
            .offset(OffsetSpecification::First)
            .build("orders")
            .await
            .unwrap();
        while let Ok(delivery) = consumer.next().await.unwrap() { // TODO: Handle errors
            if let Some(order) = delivery
                .message()
                .data()
                .map(|data| serde_json::from_str::<Order>(
                    std::str::from_utf8(&data.to_vec()).unwrap()).unwrap()
                ) {
                self.process(order);
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use database::orders::Order;

    #[async_std::test]
    async fn add_limit_order_to_empty() {
        let mut orderbook = OrderBook::new(1).await;
        assert!(orderbook.process(Order {
            id: 1,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
    }

    #[async_std::test]
    async fn add_limit_crossing_spread() {
        let mut orderbook = OrderBook::new(1).await;
        assert!(orderbook.process(Order {
            id: 1,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
    }

    #[async_std::test]
    async fn add_limit_to_existing_price_queue() {
        let mut orderbook = OrderBook::new(1).await;
        assert!(orderbook.process(Order {
            id: 1,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
    }

    #[async_std::test]
    async fn add_market_order_to_empty() {
        let mut orderbook = OrderBook::new(1).await;
        assert!(orderbook.process(Order {
            id: 1,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
    }

    #[async_std::test]
    async fn add_market_with_liquidity() {
        let mut orderbook = OrderBook::new(1).await;
        assert!(orderbook.process(Order {
            id: 1,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(8.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 5,
            sub_account_id: 1,
            price: None,
            size: 5.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 6,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 7,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
        assert!(orderbook.process(Order {
            id: 8,
            sub_account_id: 1,
            price: None,
            size: 15.0,
            side: OrderSide::Ask,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }));
    }
}