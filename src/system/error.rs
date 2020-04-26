
use futures::task::SpawnError;

pub enum Error {
    NotInitialized,
    InitializationFailed(String),
    SpawnError(SpawnError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotInitialized =>
                write!(f, "Not initialized"),
            Error::InitializationFailed(msg) =>
                write!(f, "System initialization failed: {}", msg),
            Error::SpawnError(err) =>
                write!(f, "Failed to start system: {}", err),
        }
    }
}