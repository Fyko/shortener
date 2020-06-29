use actix_web::{web, HttpRequest, HttpResponse, Responder, middleware, Result};
use actix_files::{NamedFile};
use env_logger::Env;

async fn index() -> Result<NamedFile> {
	Ok(NamedFile::open("static/index.html")?)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	use actix_web::{App, HttpServer};

	env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
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
