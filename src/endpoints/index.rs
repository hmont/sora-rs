use actix_web::{get, http::header::ContentType, HttpResponse};
use mongodb::bson::doc;
use tera::{Context, Tera};

use crate::utils::database;

#[get("/")]
async fn index() -> HttpResponse {
    let mut context = Context::new();

    context.insert("users", &database::collection("users").await.count_documents(doc!{}, None).await.unwrap());
    context.insert("images", &database::collection("uploads").await.count_documents(doc!{"type": "image"}, None).await.unwrap());
    context.insert("files", &database::collection("uploads").await.count_documents(doc!{"type": "file"}, None).await.unwrap());

    let body = Tera::one_off(include_str!("../../templates/index.html"), &context, false)
    .expect("Failed to render template"); 

    HttpResponse::Ok().insert_header(ContentType(mime::TEXT_HTML)).body(body)
}