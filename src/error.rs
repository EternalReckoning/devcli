pub enum Error {
    SystemSpawnFailed(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SystemSpawnFailed(name) =>
                write!(f, "Failed to start {} system", name),
        }
    }
}