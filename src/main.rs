mod logger;
mod webserver;
mod vecindex;

use clap::Parser;
use logger::init_logger;
use log::info;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    init_logger(&args.log_level);

    info!("Starting webserver...");
    webserver::run().await

}
