mod logger;

use clap::Parser;
use logger::init_logger;
use actix_web::{get, web, App, HttpServer, Responder};
use log::{debug,info,warn,error};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    debug!("Call greet(): Hello {}!", name);
    format!("Hello {}!", name)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    init_logger(&args.log_level);

    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
