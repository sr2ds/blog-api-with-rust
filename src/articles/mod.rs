use actix_web::{get, post, web, HttpResponse, Responder};
use bson::doc;
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client};
use std::sync::Mutex;

const MONGO_DB: &'static str = "blog";
const MONGO_COLL_LOGS: &'static str = "articles";

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
        .collection(MONGO_COLL_LOGS);

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
pub async fn article_store(data: web::Data<Mutex<Client>>) -> impl Responder {
    let collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLL_LOGS);

    let doc = doc! {"author": "David Silva", "created_at": "2021-04-13", "title": "Hello World", "content": "Isso será um post"};

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
pub async fn article_show(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Exibir o artigo: {}", id.into_inner().0))
}

#[get("/articles/{id}/remove")]
pub async fn article_remove(id: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Remover o artigo: {}", id.into_inner().0))
}
