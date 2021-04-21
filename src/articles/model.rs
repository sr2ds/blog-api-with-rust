use bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArticleRequest {
    #[serde(rename = "_id")]
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
