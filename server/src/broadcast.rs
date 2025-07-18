use std::{collections::HashMap, sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::util::errors::AppError;

pub struct Broadcaster {
    rooms: HashMap<String, Mutex<Broadcasterrooms>>,
}

#[derive(Debug, Clone, Default)]
struct Broadcasterrooms {
    clients: Vec<mpsc::Sender<sse::Event>>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Mutex<Self>> {
        let this = Arc::new(Mutex::new(Broadcaster {
            rooms: HashMap::new(),
        }));

        Self::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;

                for key in this.lock().rooms.keys() {
                    this.lock().remove_stale_clients(&key).await;
                }
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&mut self, key: &str) -> Result<(), AppError> {
        let clients = match self.rooms.get(key) {
            Some(c) => Ok(c.lock().clients.clone()),
            None => Err(AppError::InternalError),
        }?;

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        if !ok_clients.is_empty() {
            self.rooms.get(key).unwrap().lock().clients = ok_clients;
        } else {
            self.rooms.remove(key);
        }

        Ok(())
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(
        &mut self,
        key: &str,
    ) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Data::new("connected").into()).await.unwrap();

        match self.rooms.get(key) {
            Some(room) => room.lock().clients.push(tx),
            None => {
                self.rooms
                    .insert(key.to_owned(), Mutex::new(Broadcasterrooms::default()));
                self.rooms.get(key).unwrap().lock().clients.push(tx);
            }
        }

        Sse::from_infallible_receiver(rx)
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, key: &str, msg: &str) -> Result<(), AppError> {
        match self.rooms.get(key) {
            Some(room) => {
                let clients = room.lock().clients.clone();

                let send_futures = clients
                    .iter()
                    .map(|client| client.send(sse::Data::new(msg).into()));

                // try to send to all clients, ignoring failures
                // disconnected clients will get swept up by `remove_stale_clients`
                let _ = future::join_all(send_futures).await;
                Ok(())
            }
            None => Err(AppError::InternalError),
        }
    }
}
