use std::slice::Iter;
// pub use rabbitmq_stream_client::error::ProducerPublishError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stream {
    Fills,
    Orders,
}

impl Stream {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stream::Fills => "fills",
            Stream::Orders => "orders",
        }
    }

    pub fn iter() -> Iter<'static, Stream> {
        static STREAMS: [Stream; 2] = [Stream::Fills, Stream::Orders];
        STREAMS.iter()
    }
}

// pub struct Consumer {
//     consumer: RabbitMQConsumer
// }
//
// impl Consumer {
//     pub async fn consume<F>(&mut self, f: F) where F: Fn(Delivery) {
//         loop {
//             if let Some(Ok(delivery)) = self.consumer.next().await {
//                 f(delivery)
//             }
//         }
//     }
// }
//
// #[derive(Clone)]
// pub struct Producer {
//     producer: RabbitMQProducer<NoDedup>
// }
//
// impl Producer {
//     pub async fn send(&self) -> Option<ProducerPublishError> {
//         self.producer
//             .send_with_confirm(
//                 Message::builder()
//                     .body(serde_json::to_string("").unwrap()) // TODO: Dont confirm otherwise api will halt
//                     .build(),
//             )
//             .await
//             .err()
//     }
// }
//
// pub struct RabbitMQ {
//     environment: Environment
// }
//
// impl RabbitMQ {
//     pub async fn new(refresh: bool) -> Self {
//         let environment = Environment::builder()
//             .host("rabbitmq")
//             .port(5552)
//             .build()
//             .await
//             .unwrap();
//
//         if refresh {
//             let _ = Stream::iter()
//                 .map(|s| async {
//                     let _ = environment.delete_stream(s.as_str()).await; // Delete stream
//                     environment // Create stream
//                         .stream_creator()
//                         .max_length(ByteCapacity::MB(50))
//                         .max_age(Duration::new(30, 0))
//                         .create(s.as_str()).await.unwrap();
//                 });
//         }
//
//         RabbitMQ { environment }
//     }
//
//     pub async fn consumer(&self, stream: Stream) -> Consumer {
//         Consumer {
//             consumer: self.environment.consumer()
//             .offset(OffsetSpecification::First)
//             .build(stream.as_str())
//             .await
//             .unwrap()
//         }
//     }
//
//     pub async fn producer(&self, stream: Stream) -> Producer {
//         Producer {
//             producer: self.environment.producer()
//             .build(stream.as_str())
//             .await
//             .unwrap()
//         }
//     }
// }
