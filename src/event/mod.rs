mod event;

pub use event::Event;

pub type Sender = futures::channel::mpsc::Sender<Event>;
pub type Receiver = futures::channel::mpsc::Receiver<Event>;

pub fn channel() -> (Sender, Receiver) {
    futures::channel::mpsc::channel(128)
}