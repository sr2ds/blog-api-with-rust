use actix_web::{web, guard, App, HttpResponse, HttpServer, Responder};

mod db;
mod articles;

/// 404 handler
async fn p404() -> impl Responder {
    HttpResponse::BadRequest().body("Bad data")
}

/**
 * Função main com init do http server
 * @todo -> Agrupar rotas como resource
 */

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(articles::index)
            .service(articles::articles_index)
            .service(articles::article_store)
            .service(articles::article_show)
            .service(articles::article_remove)
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
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
