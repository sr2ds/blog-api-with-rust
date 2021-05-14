use actix_web::{get, guard, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};

use dotenv::dotenv;
use std::env;
use std::sync::*;

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
 * @todo -> Refactor main() removing old mongodb driver
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let mut client_options = ClientOptions::parse(&database_url).await.unwrap();
    client_options.app_name = Some("blog".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
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
