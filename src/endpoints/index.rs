use actix_web::{web, get, Result};

#[get("/")]
async fn index() -> Result<String> {
    Ok("i.henry.moe\npowered by sora\ncurrently serving stuff".to_string())
}