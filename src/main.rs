use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, http};
use env_logger::Env;
use hasura::{fetch_url};

mod hasura;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn redirect(req: HttpRequest) -> impl Responder {
    let short = req.match_info().get("name").unwrap_or("");

    let res = fetch_url(short);

    match res {
        Ok(data) => {
            let first = data.urls.first();

            if first.is_none() {
                return HttpResponse::NotFound().body("Not Fond");
            } else {
                return HttpResponse::Found().header(http::header::LOCATION, first.unwrap().long.to_string()).finish();
            }
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Internal Server Error"); 
        }
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/{name}", web::get().to(redirect))
            .default_service(web::to(|| HttpResponse::NotFound().body("Not Found")))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
