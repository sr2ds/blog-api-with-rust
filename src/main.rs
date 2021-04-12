use actix_web::{get, post, web, guard, App, HttpResponse, HttpServer, Responder};

mod db;

/**
 * Operações básicas do CRUD Articles
 * @todo -> Colocar isso em outro lugar
 */

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Retornar lista completa de artigos")
}

#[get("/articles")]
async fn articles_index() -> impl Responder {
    HttpResponse::Ok().body("Retornar lista completa de artigos")
}

#[post("/articles")]
async fn article_store() -> impl Responder {
    HttpResponse::Ok().body("Cadastro de artigo")
}

#[get("/articles/{id}")]
async fn article_show(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Exibir o artigo: {}", id.into_inner().0))
}

#[get("/articles/{id}/remove")]
async fn article_remove(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Remover o artigo: {}", id.into_inner().0))
}

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
            .service(index)
            .service(articles_index)
            .service(article_store)
            .service(article_show)
            .service(article_remove)
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
