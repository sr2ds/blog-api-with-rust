/**
* Article Model
* This is only a sample but I think that additional rules (like _all,_create,etc) are be better in one repository
*/

pub mod article {
  use std::env;
  
  use futures::stream::StreamExt;
  use serde::{Deserialize, Serialize};
  use wither::bson::{doc, oid::ObjectId};
  use wither::mongodb::Client;
  use wither::{prelude::*, Result};
  use actix_web::{web};
  
  const MONGO_DB: &'static str = "blog";
  
  #[derive(Debug, Model, Serialize, Deserialize)]
  #[model(
    index(keys = r#"doc!{"author": 1}"#, options = r#"doc!{"unique": true}"#),
    collection_name = "articles"
  )]
  
  pub struct ArticleModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
  }
  
  // @todo -> How to unify struct ArticleRequest with ArticleUpdateRequest | If possible unify with ArticleModel, why not?
  #[derive(Serialize, Deserialize)]
  pub struct ArticleRequest {
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
  }
    
  #[derive(Serialize, Deserialize)]
  pub struct ArticleUpdateRequest {
    #[serde(rename = "_id")]
    pub _id: String,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
  }
  
  pub async fn _all() -> Result<Vec<ArticleModel>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    let mut cursor = ArticleModel::find(&db, None, None).await?;
    let mut results = vec![];
    
    while let Some(result) = cursor.next().await {
      match result {
        Ok(document) => results.push(document),
        Err(_) => println!("Err"),
      }
    }
    
    Ok(results)
  }
  
  pub async fn _get_one(query: bson::Document) -> Result<Option<ArticleModel>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    let doc = ArticleModel::find_one(&db, query, None).await?;
    
    Ok(doc)
  }
  
  pub async fn _create(_article: web::Json<ArticleRequest>) -> Result<ArticleModel> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    let mut to_save = ArticleModel {
      id: None,
      author: _article.author.clone(),
      created_at: _article.created_at.clone(),
      title: _article.title.clone(),
      content: _article.content.clone(),
    };
    
    to_save.save(&db, None).await?;
    
    Ok(to_save)
  }
  
  pub async fn _update(_article:  web::Json<ArticleUpdateRequest>) -> Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    // This is bad code, how to dont pass all params? -> need improve
    let to_update = ArticleModel {
      id: serde::__private::Some(ObjectId::with_string(&_article._id.clone()).unwrap()), //  bad code
      author: _article.author.clone(),
      created_at: _article.created_at.clone(),
      title: _article.title.clone(),
      content: _article.content.clone(),
    };

    let document = doc! {
      "author": _article.author.clone(),
      "created_at": _article.created_at.clone(),
      "title": _article.title.clone(),
      "content": _article.content.clone(),
    };

    to_update.update(&db, None, doc!{"$set": document}, None).await?;   
    
    Ok(()) // it's work but is better return with document
  }
  
  pub async fn _delete(query: bson::oid::ObjectId) -> Result<mongodb::results::DeleteResult> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    // This is bad code, how to dont pass all params? -> need improve
    let to_delete = ArticleModel {
      id: serde::__private::Some(query),
      author: String::from(""),
      title: String::from(""),
      content: String::from(""),
      created_at: String::from(""),
    };
    
    let doc = ArticleModel::delete(&to_delete, &db).await?;
    
    Ok(doc)
  }
  
}
