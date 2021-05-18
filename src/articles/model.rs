/**
* Article Model
* This is only a sample but I think that additional rules (like _all,_create,etc) are be better in one repository
* @todo -> Refector _update and _delete methods
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
    println!("{:?}", doc);
    
    Ok(doc)
  }


  // @todo -> How to unify struct ArticleRequest with ArticleUpdateRequest
  #[derive(Serialize, Deserialize)]
  pub struct ArticleRequest {
      pub author: String,
      pub title: String,
      pub content: String,
      pub created_at: String,
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
  
  pub async fn _update(_article: bson::Document, query: bson::Document) -> Result<Option<ArticleModel>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    let doc = ArticleModel::find_one(&db, query, None).await?;
    println!("{:?}", doc);
    
    Ok(doc)
  }
  
  pub async fn _delete(query: bson::Document) -> Result<Option<ArticleModel>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db = Client::with_uri_str(&database_url).await?.database(MONGO_DB);
    
    let doc = ArticleModel::find_one(&db, query, None).await?;
    println!("{:?}", doc);
    
    Ok(doc)
  }

 
}
