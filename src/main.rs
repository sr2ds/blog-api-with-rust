use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

mod articles;

/// 404 handler
async fn p404() -> impl Responder {
    HttpResponse::BadRequest().body("Bad data")
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

/**
 * Função main com init do http server
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .configure(articles::init)
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
