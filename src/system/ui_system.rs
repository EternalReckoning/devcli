use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::sink::SinkExt;
use tui::Terminal;
use tui::widgets::{
    Block,
    Borders,
    List,
    Text,
};

#[cfg(feature = "term-crossterm")]
use tui::backend::CrosstermBackend as Backend;

#[cfg(feature = "term-termion")]
use termion::raw::{IntoRawMode, RawTerminal};
#[cfg(feature = "term-termion")]
use tui::backend::TermionBackend as Backend;

use crate::event::{Event, Receiver, Sender};
use super::{Error, System};

static NAME: &str = "UI";

pub struct UiSystem {
    event_rx: Receiver,
    event_tx: Option<Sender>,
}

impl UiSystem {
    pub fn new(receiver: Receiver) -> UiSystem {
        UiSystem {
            event_rx: receiver,
            event_tx: None,
        }
    }
}

#[async_trait]
impl System for UiSystem {
    fn initialize(&mut self, sender: Sender) {
        self.event_tx = Some(sender);
    }

    fn get_name(&self) -> &'static str {
        NAME
    }

    async fn run(mut self) -> Result<(), Error> {
        let mut sender = self.event_tx.ok_or(Error::NotInitialized)?;

        let mut terminal = {
            let stdout = std::io::stdout();
            #[cfg(feature = "term-termion")]
            {
                stdout = stdout.into_raw_mode()
                    .or(Err(Error::InitializationFailed(
                        "Failed to set terminal to raw mode".to_string()
                    )))?;
            }
            Terminal::new(Backend::new(stdout))
        }
            .or(Err(Error::InitializationFailed(
                "Unable to access terminal".to_string()
            )))?;
        terminal.clear();

        sender.send(Event::SystemRunning(NAME)).await;

        let mut log_items = Vec::with_capacity(64);
        loop {
            match self.event_rx.next().await {
                Some(event) => {
                    log_items.push(format!("{}", event).to_string());

                    terminal.draw(|mut f| {
                        let size = f.size();
                        let block = Block::default()
                            .title("Event Log")
                            .borders(Borders::ALL);
                        let list = List::new(
                            log_items.iter()
                                .map(|i| Text::raw(i.to_string()))
                        )
                            .block(block);
                        f.render_widget(list, size);
                    });
                },
                None => {
                    sender.send(Event::SystemStopping(NAME)).await;
                    terminal.clear();
                    return Ok(());
                },
            };
        }
    }
}