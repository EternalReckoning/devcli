mod error;
mod options;

pub use error::Error;
pub use options::Options;

use eternalreckoning_messaging::Messaging;
use eternalreckoning_messaging::Config as MessagingConfig;

pub async fn run(options: &Options) -> Result<(), Error> {
    log::info!("Connecting to {}:{}", &options.host, &options.port);
    let mut messaging = Messaging::connect(MessagingConfig {
        user: options.user.to_string(),
        password: options.password.to_string(),
        host: options.host.to_string(),
        port: options.port.to_string(),
        vhost: options.vhost.to_string(),
    })
        .await
        .map_err(|err| Error::MessagingError(err))?;

    log::debug!("Connection established");

    Ok(())
}
