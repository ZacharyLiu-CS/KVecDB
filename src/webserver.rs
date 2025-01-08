//
// webserver.rs
//
// Created by Zacharyliu-CS on 01/08/2025.
// Copyright (c) 2025 liuzhenm@mail.ustc.edu.cn.
//

use actix_web::{get, web, App, HttpServer, Responder};
use log::debug;


#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    debug!("Call greet(): Hello {}!", name);
    format!("Hello {}!", name)
}


pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
