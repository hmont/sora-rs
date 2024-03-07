use actix_web::http::header::ContentType;

use actix_web::{get, web, HttpResponse};
use mongodb::bson::doc;

use tera::{Tera, Context};

use crate::utils::database;
use crate::utils::files;

#[get("/uploads/{id}")]
async fn uploads(path: web::Path<String>) -> HttpResponse {
    let upload = database::collection("uploads").await.find_one(doc!{"id": path.into_inner()}, None).await.unwrap();

    if upload.is_none() { return HttpResponse::NotFound().body("Upload not found"); }

    let mut context = Context::new();

    context.insert("filename", upload.clone().unwrap().get_str("filename").unwrap());
    context.insert("id", upload.clone().unwrap().get_str("id").unwrap());
    context.insert("filetype", upload.clone().unwrap().get_str("type").unwrap());
    context.insert("datetime", &files::timestamp_str(upload.clone().unwrap().get_i64("timestamp").unwrap()).await);
    context.insert("uploader", upload.clone().unwrap().get_str("uploader").unwrap());

    // this was for debugging println!("Filename: {}\nID: {}\nType: {}\nDatetime: {}\nUploader: {}", upload.clone().unwrap().get_str("filename").unwrap(), upload.clone().unwrap().get_str("id").unwrap(), upload.clone().unwrap().get_str("type").unwrap(), &files::timestamp_str(upload.clone().unwrap().get_i64("timestamp").unwrap()).await, upload.clone().unwrap().get_str("uploader").unwrap());

    let body = Tera::one_off(include_str!("../../templates/upload.html"), &context, false)
    .expect("Failed to render template"); 

    HttpResponse::Ok().insert_header(ContentType(mime::TEXT_HTML)).body(body)
    
}