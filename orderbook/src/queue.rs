use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use chrono::{NaiveDateTime};
use database::orders::Order;
use database::OrderSide;

#[derive(Clone)]
struct OrderIndex {
    id: i32,
    price: f32,
    timestamp: NaiveDateTime,
    side: OrderSide,
}

impl Ord for OrderIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.price < other.price {
            match self.side {
                OrderSide::Buy | OrderSide::Bid | OrderSide::Long => Ordering::Less,
                OrderSide::Sell | OrderSide::Ask | OrderSide::Short => Ordering::Greater,
            }
        } else if self.price > other.price {
            match self.side {
                OrderSide::Buy | OrderSide::Bid | OrderSide::Long => Ordering::Greater,
                OrderSide::Sell | OrderSide::Ask | OrderSide::Short => Ordering::Less,
            }
        } else {
            other.timestamp.cmp(&self.timestamp)
        }
    }
}

impl PartialOrd for OrderIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderIndex {
    fn eq(&self, other: &Self) -> bool {
        if self.price > other.price || self.price < other.price {
            false
        } else {
            self.timestamp == other.timestamp
        }
    }
}

impl Eq for OrderIndex {}

pub struct Queue {
    idx_queue: BinaryHeap<OrderIndex>, // Use Option in order to replace heap in mutable borrow
    orders: HashMap<i32, Order>,
}

impl Queue {
    pub fn new(capacity: usize) -> Self {
        Queue {
            idx_queue: BinaryHeap::with_capacity(capacity),
            orders: HashMap::with_capacity(capacity),
        }
    }

    pub fn peek(&mut self) -> Option<&Order> {
        let id = self.idx_queue.peek()?.id;
        // self.orders.get(&id)
        if self.orders.contains_key(&id) {
            self.orders.get(&id)
        } else {
            self.idx_queue.pop()?;
            self.peek()
        }
    }

    pub fn pop(&mut self) -> Option<Order> {
        let id = self.idx_queue.pop()?.id;
        if self.orders.contains_key(&id) {
            self.orders.remove(&id)
        } else {
            self.pop()
        }
    }

    pub fn insert(&mut self, order: Order) -> bool {
        if self.orders.contains_key(&order.id) {
            return false;
        }
        self.idx_queue.push(OrderIndex {
            id: order.id,
            price: order.price.unwrap(),
            timestamp: order.open_at,
            side: order.side.clone(),
        });
        self.orders.insert(order.id, order);
        true
    }

    pub fn cancel(&mut self, id: i32) -> bool {
        match self.orders.remove(&id) {
            Some(_) => {
                self.idx_queue.retain(|o| o.id != id);
                true
            },
            None => false
        }
    }

    pub fn amend(&mut self, id: i32, size: f32) -> bool {
        if let Some(order) = self.orders.get_mut(&id) {
            order.size = size;
            true
        } else {
            false
        }
    }

    pub fn modify_tob(&mut self, size: f32) -> bool {
        if let Some(order_index) = self.idx_queue.peek() {
            if let Some(order) = self.orders.get_mut(&order_index.id) {
                order.size = size; // TODO: Rather insert than modify inplace
                return true;
            }
        }
        false
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use chrono::Utc;
//     use database::{OrderSide, OrderType};
//
//     fn bids() -> Queue {
//         let mut queue = Queue::new(10);
//         assert!(queue.insert(Order {
//             id: 1,
//             sub_account_id: 1,
//             price: Some(1.01),
//             size: 10.0,
//             side: OrderSide::Bid,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         assert!(queue.insert(Order {
//             id: 2,
//             sub_account_id: 1,
//             price: Some(1.02),
//             size: 10.0,
//             side: OrderSide::Bid,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         assert!(queue.insert(Order {
//             id: 3,
//             sub_account_id: 1,
//             price: Some(1.02),
//             size: 10.0,
//             side: OrderSide::Bid,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         queue
//     }
//
//     fn asks() -> Queue {
//         let mut queue = Queue::new(10);
//         assert!(queue.insert(Order {
//             id: 1,
//             sub_account_id: 1,
//             price: Some(1.01),
//             size: 10.0,
//             side: OrderSide::Ask,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         assert!(queue.insert(Order {
//             id: 2,
//             sub_account_id: 1,
//             price: Some(1.02),
//             size: 10.0,
//             side: OrderSide::Ask,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         assert!(queue.insert(Order {
//             id: 3,
//             sub_account_id: 1,
//             price: Some(1.01),
//             size: 10.0,
//             side: OrderSide::Ask,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         queue
//     }
//
//     #[test]
//     fn insert_unique() {
//         let mut queue = Queue::new(10);
//         assert!(queue.peek().is_none());
//         assert!(queue.insert(Order { // Insert unique with success
//             id: 1,
//             sub_account_id: 1,
//             price: Some(1.01),
//             size: 10.0,
//             side: OrderSide::Bid,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//         assert!(!queue.insert(Order { // Insert existing with failure
//             id: 1,
//             sub_account_id: 1,
//             price: Some(1.01),
//             size: 10.0,
//             side: OrderSide::Bid,
//             r#type: OrderType::Limit,
//             open_at: Utc::now().naive_utc(),
//         }));
//     }
//
//
//     #[test]
//     fn bids_ordering() {
//         let mut queue = bids();
//         assert_eq!(queue.pop().unwrap().id, 2);
//         assert_eq!(queue.pop().unwrap().id, 3);
//         assert_eq!(queue.pop().unwrap().id, 1);
//     }
//
//     #[test]
//     fn queue_operations_ordering_ask() {
//         let mut queue = asks();
//         assert_eq!(queue.pop().unwrap().id, 1);
//         assert_eq!(queue.pop().unwrap().id, 3);
//         assert_eq!(queue.pop().unwrap().id, 2);
//     }
//
//     #[test]
//     fn modify_tob() {
//         let mut queue = bids();
//         assert!(queue.modify_tob(5.0));
//         assert_eq!(queue.pop().unwrap().size, 5.0);
//     }
//
//     #[test]
//     fn amend() {
//         let mut queue = asks();
//         assert!(queue.amend(1,1.0));
//         assert!(queue.amend(2,2.0));
//         assert!(queue.amend(3,3.0));
//
//         assert_eq!(queue.pop().unwrap().size, 1.0);
//         assert_eq!(queue.pop().unwrap().size, 3.0);
//         assert_eq!(queue.pop().unwrap().size, 2.0);
//     }
//
//     #[test]
//     fn cancel_bid() {
//         let mut queue = bids();
//         queue.cancel(1);
//         assert_eq!(queue.pop().unwrap().id, 2);
//         assert_eq!(queue.pop().unwrap().id, 3);
//     }
//
//     #[test]
//     fn cancel_ask() {
//         let mut queue = asks();
//         queue.cancel(1);
//         assert_eq!(queue.pop().unwrap().id, 3);
//         assert_eq!(queue.pop().unwrap().id, 2);
//     }
// }