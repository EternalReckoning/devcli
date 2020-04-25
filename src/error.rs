use eternalreckoning_messaging::Error as MessagingError;

pub enum Error {
    MessagingError(MessagingError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::MessagingError(err) =>
                write!(f, "Messaging: {}", err),
        }
    }
}

impl std::convert::From<MessagingError> for Error {
    fn from(error: MessagingError) -> Self {
        Error::MessagingError(error)
    }
}
