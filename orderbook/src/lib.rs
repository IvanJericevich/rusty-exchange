#![feature(binary_heap_retain)]

use std::collections::HashMap;

use async_recursion::async_recursion;
use chrono::Utc;

use common::rabbitmq::{Producer, RabbitMQ, Stream};
use database::{OrderSide, OrderType};
use database::fills::Fill;
use database::orders::Order;

use crate::queue::Queue;

mod queue;

const QUEUE_CAPACITY: usize = 500;

pub struct OrderBook {
    id: i32,
    bids: Queue,
    asks: Queue,
    producer: Option<HashMap<Stream, Producer>>,
}

impl OrderBook {
    pub async fn new(market_id: i32) -> Self {
        let producer = if !cfg!(test) {
            let rabbitmq = RabbitMQ::new(false).await;
            Some(HashMap::from([
                (Stream::Fills, rabbitmq.producer(Stream::Fills).await),
                (Stream::Orders, rabbitmq.producer(Stream::Orders).await),
            ]))
        } else {
            None
        };
        OrderBook {
            id: market_id,
            bids: Queue::new(QUEUE_CAPACITY),
            asks: Queue::new(QUEUE_CAPACITY),
            producer,
        }
    }

    async fn process(&mut self, order: Order) -> bool {
        match order.r#type {
            OrderType::Limit => self.process_limit(order).await,
            OrderType::Market => self.process_market(order).await,
        }
    }

    #[async_recursion]
    async fn process_limit(&mut self, order: Order) -> bool {
        if let Some(contra_order) = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => self.asks.peek().cloned(),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => self.bids.peek().cloned(),
        } {
            if match order.side {
                OrderSide::Buy | OrderSide::Bid | OrderSide::Long => {
                    order.price >= contra_order.price
                }
                OrderSide::Sell | OrderSide::Ask | OrderSide::Short => {
                    order.price <= contra_order.price
                }
            } {
                let mut order = order; // Take the previous value out of scope
                if !self.cross(&mut order, contra_order).await {
                    self.process_limit(order).await
                } else {
                    self.publish_order(order).await; // Publish order after its been fully processed
                    true
                }
            } else {
                self.store(order)
            }
        } else {
            self.store(order)
        }
    }

    #[async_recursion]
    async fn process_market(&mut self, order: Order) -> bool {
        if let Some(contra_order) = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => self.asks.peek().cloned(),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => self.bids.peek().cloned(),
        } {
            let mut order = order;
            if !self.cross(&mut order, contra_order).await {
                self.process_market(order).await;
            }
        }
        true
    }

    fn store(&mut self, order: Order) -> bool {
        match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => self.bids.insert(order),
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => self.asks.insert(order),
        }
    }

    async fn cross(&mut self, order: &mut Order, contra_order: Order) -> bool {
        self.publish_fill(
            contra_order.price.unwrap(),
            f32::min(order.size, contra_order.size),
            contra_order.side,
            contra_order.r#type,
            contra_order.sub_account_id,
            contra_order.id,
        ).await;
        self.publish_fill(
            contra_order.price.unwrap(),
            f32::min(order.size, contra_order.size),
            order.side.clone(),
            order.r#type.clone(),
            order.sub_account_id,
            order.id,
        ).await;
        let contra_queue = match order.side {
            OrderSide::Buy | OrderSide::Bid | OrderSide::Long => &mut self.asks,
            OrderSide::Sell | OrderSide::Ask | OrderSide::Short => &mut self.bids,
        };
        if order.size < contra_order.size {
            // Modify the contra order
            contra_queue.modify_tob(contra_order.size - order.size);
            true
        } else if order.size > contra_order.size {
            // Modify the submitted order
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

    async fn publish_fill(
        &mut self,
        price: f32,
        size: f32,
        side: OrderSide,
        r#type: OrderType,
        sub_account_id: i32,
        order_id: i32,
    ) {
        if let Some(producer) = &self.producer {
            let _ = producer.get(&Stream::Fills)
                .unwrap()
                .send(&Fill {
                    price,
                    size,
                    quote_size: price * size,
                    side,
                    r#type,
                    created_at: Utc::now().naive_utc(),
                    sub_account_id,
                    market_id: self.id,
                    order_id,
                })
                .await;
        }
    }

    async fn publish_order(
        &mut self,
        order: Order,
    ) -> bool {
        if let Some(producer) = &self.producer {
            let _ = producer
                .get(&Stream::Orders)
                .unwrap()
                .send(&order)
                .await;
        }
        true
    }

    pub async fn run(market_id: i32) {
        let producer = if !cfg!(test) {
            let rabbitmq = RabbitMQ::new(false).await;
            Some(HashMap::from([
                (Stream::Fills, rabbitmq.producer(Stream::Fills).await),
                (Stream::OpenOrders, rabbitmq.producer(Stream::OpenOrders).await),
            ]))
        } else {
            None
        };
        let mut orderbook = OrderBook {
            id: market_id,
            bids: Queue::new(QUEUE_CAPACITY),
            asks: Queue::new(QUEUE_CAPACITY),
            producer,
        };
        let mut consumer = RabbitMQ::new(false).await
            .consumer(Stream::Orders).await;
        loop {
            if let Some(order) = consumer.next::<Order>().await {
                orderbook.process(order).await;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use database::orders::Order;

    use super::*;

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
        }).await);
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
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
        }).await);
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(11.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
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
        }).await);
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
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
        }).await);
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }).await);
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
        }).await);
        assert!(orderbook.process(Order {
            id: 2,
            sub_account_id: 1,
            price: Some(10.0),
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 3,
            sub_account_id: 1,
            price: Some(9.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 4,
            sub_account_id: 1,
            price: Some(8.0),
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Limit,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 5,
            sub_account_id: 1,
            price: None,
            size: 5.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 6,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Bid,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 7,
            sub_account_id: 1,
            price: None,
            size: 10.0,
            side: OrderSide::Ask,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }).await);
        assert!(orderbook.process(Order {
            id: 8,
            sub_account_id: 1,
            price: None,
            size: 15.0,
            side: OrderSide::Ask,
            r#type: OrderType::Market,
            open_at: Utc::now().naive_utc(),
        }).await);
    }
}
