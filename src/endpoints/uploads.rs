use actix_web::{web, get, Result};
use serde::Deserialize;

use crate::utils::users;

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/uploads/{id}")]
async fn uploads(info: web::Query<Info>) -> Result<String> {
    
    if user.is_none() {
        return Ok("Unauthorized".to_string());
    } else {
        return Ok("Authorized".to_string());
    }
}