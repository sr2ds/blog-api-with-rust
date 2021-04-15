use actix_web::{get, post, web, HttpResponse, Responder};
use bson::doc;
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client, bson};
use std::sync::Mutex;

use bson::oid::ObjectId;
use bson::Bson;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{Result, Value};

const MONGO_DB: &'static str = "blog";
const MONGO_COLLECTION_ARTICLES: &'static str = "articles";

#[derive(Serialize, Deserialize)]
pub struct NewArticle {
    #[serde(rename = "_id")]
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

/**
 * Operações básicas do CRUD Articles
 * @todo -> Separar lógicas de DB
*/

#[get("/articles")]
pub async fn articles_index(data: web::Data<Mutex<Client>>) -> impl Responder {
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
pub async fn article_store(data: web::Data<Mutex<Client>>, new_article: web::Json<NewArticle>) -> impl Responder {
    let collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLLECTION_ARTICLES);

    let doc = doc! {"author": &new_article.author, "created_at": &new_article.created_at, "title": &new_article.title, "content": &new_article.content};

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
pub async fn article_show(data: web::Data<Mutex<Client>>, id: web::Path<(String,)>) -> impl Responder {
    
    let collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLLECTION_ARTICLES);
    
    // // let article = collection.find_one(doc! { "_id": Bson::ObjectId(ObjectId::with_string("6078951e00c0d4aa00ea9350").unwrap()) }, None).await;
    // // let body = serde_json::to_string(&article).unwrap();

    // HttpResponse::Created().json(body)
    HttpResponse::Ok().body("Hello!!")
}

#[get("/articles/{id}/remove")]
pub async fn article_remove(data: web::Data<Mutex<Client>>, id: web::Path<(String,)>) -> impl Responder {

    // let collection = data
    // .lock()
    // .unwrap()
    // .database(MONGO_DB)
    // .collection(MONGO_COLLECTION_ARTICLES);
    
    // let article = collection.remove(doc! { "_id": Bson::ObjectId(ObjectId::with_string("6078951e00c0d4aa00ea9350").unwrap()) }, None).await;
    // let body = serde_json::to_string(&article).unwrap();

    // HttpResponse::Ok().body(format!("Remover o artigo: {}", id.into_inner().0))
    HttpResponse::Ok().body("Hello!!")

}
