use std::slice::Iter;
use std::time::Duration;

pub use futures::StreamExt;
use rabbitmq_stream_client::{Consumer as StreamConsumer, Environment, NoDedup, Producer as StreamProducer};
pub use rabbitmq_stream_client::error::ProducerPublishError;
use rabbitmq_stream_client::types::{ByteCapacity, Message, OffsetSpecification};

use crate::util;

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

pub struct Consumer {
    consumer: StreamConsumer,
}

impl Consumer {
    pub async fn next<T>(&mut self) -> Option<T> where T: serde::de::DeserializeOwned {
        if let Some(Ok(delivery)) = self.consumer.next().await {
            delivery
                .message()
                .data()
                .map(|data|
                    serde_json::from_str::<T>(
                        std::str::from_utf8(data).unwrap()
                    ).unwrap()
                )
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Producer {
    producer: StreamProducer<NoDedup>,
}

impl Producer {
    pub async fn send<T>(&self, msg: &T) -> Option<ProducerPublishError> where T: serde::Serialize {
        self.producer
            .send_with_confirm(
                Message::builder()
                    .body(serde_json::to_string(msg).unwrap())
                    .build(),
            )
            .await
            .err()
    }
}

pub struct RabbitMQ {
    environment: Environment,
}

impl RabbitMQ {
    pub async fn new(refresh: bool) -> Self {
        let environment = Environment::builder()
            .host(if util::is_running_in_container() {
                "rabbitmq"
            } else {
                "localhost"
            })
            .port(5552)
            .build()
            .await
            .unwrap();

        if refresh {
            let _ = Stream::iter()
                .map(|s| async {
                    let _ = environment.delete_stream(s.as_str()).await; // Delete stream
                    environment // Create stream
                        .stream_creator()
                        .max_length(ByteCapacity::MB(50))
                        .max_age(Duration::new(30, 0))
                        .create(s.as_str()).await.unwrap();
                });
        }

        RabbitMQ { environment }
    }

    pub async fn consumer(&self, stream: Stream) -> Consumer {
        Consumer {
            consumer: self.environment.consumer()
                .offset(OffsetSpecification::First)
                .build(stream.as_str())
                .await
                .unwrap()
        }
    }

    pub async fn producer(&self, stream: Stream) -> Producer {
        Producer {
            producer: self.environment.producer()
                .build(stream.as_str())
                .await
                .unwrap()
        }
    }
}
