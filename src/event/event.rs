use crate::system::Error as SystemError;

pub enum Event {
    // Lifecycle
    SystemInitializing(&'static str),
    SystemRunning(&'static str),
    SystemStopping(&'static str),
    SystemStopped(&'static str),

    // Errors
    SystemError(&'static str, SystemError),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::SystemInitializing(name) =>
                write!(f, "{}: Initialization ongoing", name),
            Event::SystemRunning(name) =>
                write!(f, "{}: System online", name),
            Event::SystemStopping(name) =>
                write!(f, "{}: System preparing to stop", name),
            Event::SystemStopped(name) =>
                write!(f, "{}: System terminated", name),

            Event::SystemError(name, err) =>
                write!(f, "{}: Error! {}", name, err),
        }
    }
}