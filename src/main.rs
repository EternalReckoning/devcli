use structopt::StructOpt;

use eternalreckoning_devcli::{Options, Error, run};

fn main() {
    let env = env_logger::Env::default()
        .filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let opts = Options::from_args();

    run(&opts).unwrap_or_else(fatal_error);
}

fn fatal_error(error: Error) {
    log::error!("Fatal Error!");
    log::error!("{}", error);
}
