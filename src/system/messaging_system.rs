use async_trait::async_trait;
use futures::sink::SinkExt;

pub use eternalreckoning_messaging::Config as MessagingConfig;
use eternalreckoning_messaging::Messaging;

use crate::event::{Event, Sender};
use super::{Error, System};

static NAME: &str = "Messaging";

pub struct MessagingSystem {
    event_tx: Option<Sender>,
    config: MessagingConfig,
}

impl MessagingSystem {
    pub fn new(config: MessagingConfig) -> MessagingSystem {
        MessagingSystem {
            event_tx: None,
            config,
        }
    }
}

#[async_trait]
impl System for MessagingSystem {
    fn initialize(&mut self, sender: Sender) {
        self.event_tx = Some(sender);
    }

    fn get_name(&self) -> &'static str {
        NAME
    }

    async fn run(self) -> Result<(), Error> {
        let mut sender = self.event_tx.ok_or(Error::NotInitialized)?;

        let messaging = Messaging::connect(self.config).await
            .map_err(|err| Error::InitializationFailed(
                format!("{}", err).to_string()
            ))?;

        sender.send(Event::SystemRunning(NAME)).await;

        Ok(())
    }
}