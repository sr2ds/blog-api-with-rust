use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use bson::doc;
use mongodb::{bson};
use bson::oid::ObjectId;

use crate::articles::{article};

/**
* Crud Basic Operations
*/

#[get("/articles")]
pub async fn index() -> impl Responder {
    let articles = article::_all().await;
    
    match articles {
        Ok(articles) => HttpResponse::Ok().json(articles),
        _ => HttpResponse::InternalServerError().finish()
    }
}

#[get("/articles/{id}")]
pub async fn show(id: web::Path<String>) -> impl Responder {
    let object_id = ObjectId::with_string(&id).unwrap();
    let article = article::_get_one(doc! { "_id": object_id }).await;
    
    match article {
        Ok(article) => HttpResponse::Ok().json(article),
        _ => HttpResponse::InternalServerError().finish()
    }
}

#[post("/articles")]
pub async fn store(article: web::Json<article::ArticleRequest>) -> impl Responder {
    let stored = article::_create(article).await;
    
    match stored {
        Ok(stored) => HttpResponse::Ok().json(stored),
        _ => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/articles/{id}")]
pub async fn remove(id: web::Path<String>) -> impl Responder {
    let object_id = match ObjectId::with_string(&id) {
        Ok(obj) => obj,
        Err(_) => ObjectId::with_string("").unwrap() // This is bad code -> need improve
    };

    let deleted = article::_delete(object_id).await;
    match deleted {
        Ok(deleted) => HttpResponse::Ok().json(deleted),
        _ => HttpResponse::InternalServerError().finish()
    }
}

#[put("/articles/{id}")]
pub async fn update(article: web::Json<article::ArticleUpdateRequest>) -> impl Responder {
    let updated = article::_update(article).await;
    
    match updated {
        Ok(updated) => HttpResponse::Ok().json(updated),
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(remove);
    cfg.service(update);
}