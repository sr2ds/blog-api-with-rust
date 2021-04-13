use actix_web::{get, post, web, HttpResponse, Responder};

/**
 * Operações básicas do CRUD Articles
*/

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Retornar lista completa de artigos")
}

#[get("/articles")]
pub async fn articles_index() -> impl Responder {
    HttpResponse::Ok().body("Retornar lista completa de artigos")
}

#[post("/articles")]
pub async fn article_store() -> impl Responder {
    HttpResponse::Ok().body("Cadastro de artigo")
}

#[get("/articles/{id}")]
pub async fn article_show(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Exibir o artigo: {}", id.into_inner().0))
}

#[get("/articles/{id}/remove")]
pub async fn article_remove(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Remover o artigo: {}", id.into_inner().0))
}
