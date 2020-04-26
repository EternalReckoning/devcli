use futures::task::SpawnExt;
use futures::executor::{LocalPool, LocalSpawner};
use futures::sink::SinkExt;

use crate::Error;
use crate::event::{
    Event,
    Sender as EventSender,
};
use super::System;

pub struct Pool {
    event_tx: EventSender,
    pool: LocalPool,
    spawn: LocalSpawner,
}

impl Pool {
    pub fn new(event_sender: EventSender) -> Pool {
        let pool = LocalPool::new();
        let spawn = pool.spawner();

        Pool {
            event_tx: event_sender,
            pool,
            spawn,
        }
    }

    pub fn spawn<T>(&self, mut system: T) -> Result<(), Error>
    where
        T: System + Send + 'static
    {
        let mut sender = self.event_tx.clone();
        let name = system.get_name();

        self.spawn.spawn(async move {
            sender.send(Event::SystemInitializing(name)).await;
            system.initialize(sender.clone());

            if let Err(err) = system.run().await {
                sender.send(Event::SystemError(name, err)).await;
            }

            sender.send(Event::SystemStopped(name)).await;
        })
            .map_err(|_| Error::SystemSpawnFailed(name))
    }

    pub fn run(mut self) {
        self.pool.run();
    }
}