mod error;
mod event;
mod options;
mod system;

pub use error::Error;
pub use options::Options;

pub fn run(options: &Options) -> Result<(), Error> {
    let (event_tx, event_rx) = event::channel();
    let pool = system::Pool::new(event_tx);

    let ui = system::UiSystem::new(event_rx);
    pool.spawn(ui)?;

    let messaging = system::MessagingSystem::new(
        system::messaging_system::MessagingConfig {
            user: options.user.to_string(),
            password: options.password.to_string(),
            host: options.host.to_string(),
            port: options.port.to_string(),
            vhost: options.vhost.to_string(),
        }
    );
    pool.spawn(messaging)?;

    pool.run();

    Ok(())
}