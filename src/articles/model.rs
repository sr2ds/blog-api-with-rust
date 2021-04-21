use bson::doc;
use serde::{Deserialize, Serialize};

/**
 * @todo -> trazer regras de consulta pra cá
 * @todo -> collection e dbname no .env
 */

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

#[derive(Serialize)]
pub struct Article {
    // pub id: i32,
    pub author: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

impl Article {
    pub async fn _all() {
        
    }
    pub async fn _create() {
        
    }
    pub async fn _update() {
        
    }
    pub async fn _delete() {
        
    }
}
