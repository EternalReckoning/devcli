use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use structopt::StructOpt;

use eternalreckoning_devcli::{Options, Error, run};

fn main() {
    let env = env_logger::Env::default()
        .filter_or("LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    let opts = Options::from_args();

    let mut pool = LocalPool::new();
    let spawn = pool.spawner();

    spawn.spawn_local(async move {
        run(&opts)
            .await
            .unwrap_or_else(fatal_error)
        }
    )
        .expect("Unable to start asynchronous execution");

    pool.run();
}

fn fatal_error(error: Error) {
    log::error!("Fatal Error!");
    log::error!("{}", error);
}
