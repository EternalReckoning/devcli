use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(default_value = "guest", long, short)]
    pub user: String,
    #[structopt(default_value = "guest", long, short)]
    pub password: String,
    #[structopt(default_value = "localhost", long, short = "H")]
    pub host: String,
    #[structopt(default_value = "5672", long, short = "P")]
    pub port: String,
    #[structopt(name = "virtual host", default_value = "/", long = "vhost")]
    pub vhost: String,
}
