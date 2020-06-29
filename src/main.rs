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

// async fn fetch_from_redis(key: String, redis: web::Data<PairedConnection>) -> String {
//     let res: String = redis.send(resp_array!["GET", key]).await.expect("");
//     return res;
// }

// async fn fetch_from_db(key: String, db: &web::Data<pg::DbConnection>) {
//     use schema::urls::dsl::*;

//     let x = urls.load::<models::ShortenedURL>(&db);
// }

// async fn redirect(req: HttpRequest, db: web::Data<pg::DbPool>, redis: web::Data<PairedConnection>) -> impl Responder {
//     let short = req.match_info().get("name").unwrap_or("");
//     format!("Hello {}!", &short);

//     // if short.is_empty() {
//     //     format!("Not Found.");
//     // }

//     // let cached = fetch_from_redis(short.to_string(), redis.clone()).await;

//     // format!("{}", &cached);
// }

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	env_logger::from_env(Env::default().default_filter_or("info")).init();

    let addr = env::var("REDIS_ADDRESS")
            .expect("127.0.0.1:6379")
            .parse()
            .unwrap();

    let redis_conn = client::paired_connect(&addr).await.expect("Failed to open connection to Redis.");

    HttpServer::new(move || {
        let pg_pool = pg::init_pool();
        let redis = redis_conn.clone();

        App::new()
            .data(redis)
            .data(pg_pool)
			.wrap(middleware::Logger::default())
			.route("/", web::get().to(index))
            .route("/{name}", web::get().to(greet))
			.default_service(web::to(|| HttpResponse::NotFound().body("Not Found") ))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
