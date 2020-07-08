#[macro_use]
extern crate diesel;

use redis_async::{client, resp_array};
use redis_async::client::PairedConnection;
use actix_web::{web, HttpRequest, HttpResponse, Responder, middleware, Result, App, HttpServer};
use actix_files::{NamedFile};
// use diesel::prelude::*;
use env_logger::Env;
use std::env;

mod models;
mod schema;
mod pg;

async fn index() -> Result<NamedFile> {
	Ok(NamedFile::open("static/index.html")?)
}

async fn redirect(req: HttpRequest) -> impl Responder {
    let short = req.match_info().get("name").unwrap_or("");
    return format!("Hello {}!", &short);
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
			.wrap(middleware::Logger::default())
			.route("/", web::get().to(index))
            .route("/{name}", web::get().to(greet))
			.default_service(web::to(|| HttpResponse::NotFound().body("Not Found") ))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
