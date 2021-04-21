use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use bson::doc;
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client, bson};
use std::sync::Mutex;

use bson::oid::ObjectId;
use bson::Bson;

const MONGO_DB: &'static str = "blog";
const MONGO_COLLECTION_ARTICLES: &'static str = "articles";

use crate::articles::{Article, ArticleUpdateRequest, ArticleRequest};

/**
 * Operações básicas do CRUD Articles
 */

#[get("/articles")]
pub async fn index(data: web::Data<Mutex<Client>>) -> impl Responder {
    let collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION_ARTICLES);

    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();

    let mut cursor = collection.find(filter, find_options).await.unwrap();
    let mut results = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}

#[post("/articles")]
pub async fn store(data: web::Data<Mutex<Client>>, article: web::Json<ArticleRequest>) -> impl Responder {
    let collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION_ARTICLES);

    let doc = doc! {"author": &article.author, "created_at": &article.created_at, "title": &article.title, "content": &article.content};

    match collection.insert_one(doc, None).await {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {}", new_id);
            }
            return HttpResponse::Created().json(db_result.inserted_id);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[get("/articles/{id}")]
pub async fn show(data: web::Data<Mutex<Client>>, id: web::Path<String>) -> impl Responder {
    let collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLLECTION_ARTICLES);

    match collection.find_one(doc! { "_id": Bson::ObjectId(ObjectId::with_string(&id).unwrap()) }, None).await {
        Ok(art) => HttpResponse::Ok().json(art),
        Err(_err) => HttpResponse::InternalServerError().finish()
    }
}

#[delete("/articles/{id}")]
pub async fn remove(data: web::Data<Mutex<Client>>, id: web::Path<String>) -> impl Responder {
    let collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLLECTION_ARTICLES);

    match collection.delete_one(doc! { "_id": Bson::ObjectId(ObjectId::with_string(&id).unwrap()) }, None).await {
        Ok(art) => HttpResponse::Ok().json(art),
        Err(_err) => HttpResponse::InternalServerError().finish()
    }
}

// @todo -> Conseguir pegar o path junto com o body, só consegui pegar um dos dois
#[put("/articles/{id}")]
pub async fn update(data: web::Data<Mutex<Client>>, article: web::Json<ArticleUpdateRequest>) -> impl Responder {
    let collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLLECTION_ARTICLES);

    let update = doc!{"$set": doc! {"author": &article.author, "created_at": &article.created_at, "title": &article.title, "content": &article.content}};
    let query = doc! { "_id": Bson::ObjectId(ObjectId::with_string(&article._id).unwrap()) };

    match collection.update_one(query, update, None).await {
        Ok(art) => HttpResponse::Ok().json(art),
        Err(_err) => HttpResponse::InternalServerError().finish()
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(store);
    cfg.service(show);
    cfg.service(remove);
    cfg.service(update);
}