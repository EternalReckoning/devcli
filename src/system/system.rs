use async_trait::async_trait;

use crate::event::Sender as EventSender;
use super::Error;

#[async_trait]
pub trait System {
    fn initialize(&mut self, sender: EventSender);
    fn get_name(&self) -> &'static str;

    async fn run(self) -> Result<(), Error>;
}