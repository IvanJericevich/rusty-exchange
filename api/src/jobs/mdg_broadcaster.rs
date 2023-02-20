use std::{env, sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::__reexports::futures_util::future;
use actix_web_lab::sse::{self, ChannelStream, Sse};
use parking_lot::Mutex;

use common::rabbitmq::{RabbitMQ, Stream};
use database::fills::Fill;

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
        let disable_rabbitmq = env::args().any(|arg| arg.to_lowercase() == "disable_rabbitmq");

        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        if !cfg!(test) && !disable_rabbitmq {
            Broadcaster::spawn_fills_broadcaster(Arc::clone(&this));
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
            { ok_clients.push((stream, client.clone())); }
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

    fn spawn_fills_broadcaster(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            // Spawns a future on the current thread as a new task
            let mut consumer = RabbitMQ::new(false).await
                .consumer(Stream::Fills).await;
            loop {
                if let Some(fill) = consumer.next::<Fill>().await {
                    this.broadcast(
                        serde_json::to_string(&fill).unwrap().as_str(),
                        Stream::Fills,
                    ).await;
                }
            }
        });
    }
}
