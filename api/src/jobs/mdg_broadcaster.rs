use futures::StreamExt;
use rabbitmq_stream_client::types::OffsetSpecification;
use rabbitmq_stream_client::Environment;

use std::{env, sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::__reexports::futures_util::future;
use actix_web_lab::sse::{self, ChannelStream, Sse};

use crate::models::Stream;
use parking_lot::Mutex;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<(Stream, sse::Sender)>,
}

impl Broadcaster {
    /// Constructs a single new broadcaster that handles all clients and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let enable_rabbitmq = env::args().any(|arg| arg.to_lowercase() == "enable_rabbitmq");

        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        if !cfg!(test) && enable_rabbitmq {
            let _ = Stream::iter()
                .map(|s| Broadcaster::spawn_broadcaster(Arc::clone(&this), s.clone()));
        }

        this
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for (stream, client) in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push((stream, client.clone()));
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self, stream: Stream) -> Sse<ChannelStream> {
        let (tx, rx) = sse::channel(5);

        tx.send(sse::Data::new("connected")).await.unwrap();

        self.inner.lock().clients.push((stream, tx));

        rx
    }

    /// Broadcasts `msg` to all clients.
    async fn broadcast(&self, msg: &str, stream: Stream) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .filter(|(s, _)| *s == stream)
            .map(|(_, client)| client.send(sse::Data::new(msg)));

        // Try to send to all clients, ignoring failures since
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            // Spawns a future on the current thread as a new task
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    fn spawn_broadcaster(this: Arc<Self>, stream: Stream) {
        actix_web::rt::spawn(async move {
            // Spawns a future on the current thread as a new task
            let mut consumer = Environment::builder()
                .host("localhost")
                .port(5552)
                .build()
                .await
                .unwrap()
                .consumer()
                .offset(OffsetSpecification::First)
                .build(stream.as_str())
                .await
                .unwrap();
            loop {
                if let Some(Ok(delivery)) = consumer.next().await {
                    if let Some(fill) = delivery
                        .message()
                        .data()
                        .map(|data| std::str::from_utf8(data).unwrap())
                    {
                        this.broadcast(fill, stream.clone()).await;
                    }
                }
            }
        });
    }
}
