use std::io::{Read, Write};

use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
    },
    Multipart,
};
use futures::FutureExt;
use mongodb::bson::doc;

use std::path::Path;
use std::fs::File;

use actix_web::{middleware, post, web, http, Error, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};

use crate::utils::{database, users};
use crate::utils::files;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Serialize)]
#[allow(unused)]
struct Response {
    status: i64,
    reason: Option<String>,
    id: Option<String>
}

#[derive(Deserialize)]
struct Info {
    api_key: String,
    r#type: String
}

#[post("/upload")]
async fn upload(info: web::Query<Info>, MultipartForm(form): MultipartForm<UploadForm>) -> HttpResponse {
    let mut filename = "".to_string();
    let mut ext = "".to_string();

    let mut filesize: i64 = 0;

    let user = users::get_user(info.api_key.clone()).await;

    if user.is_none() { 
        let obj = Response {
            status: 403,
            reason: Some("Invalid API key".to_string()),
            id: None
        };

        return HttpResponse::Forbidden().body(serde_json::to_string(&obj).unwrap())
    }

    if !(info.r#type.eq("file") || info.r#type.eq("image")) { 
        let obj = Response {
            status: 403,
            reason: Some("Invalid type".to_string()),
            id: None
        };

        return HttpResponse::Forbidden().body(serde_json::to_string(&obj).unwrap())
    }

    if form.files.len() > 1 { 
        let obj = Response {
            status: 403,
            reason: Some("One file at a time please".to_string()),
            id: None
        }; 

        println!("{}", form.files.len());

        return HttpResponse::Forbidden().body(serde_json::to_string(&obj).unwrap())
    }

    for f in form.files {
        filename = f.file_name.unwrap();
        ext = Path::new(&filename).extension().unwrap().to_str().unwrap().to_string();

        filename = files::gen_filename(8).await;

        while Path::new(&format!("./files/{}.{}", filename.clone(), ext)).exists()
        {
            filename = files::gen_filename(8).await;
        }

        let path = format!("./files/{}.{}", filename, ext);

        f.file.persist(path).unwrap();
    }

    let obj = Response {
        status: 200,
        reason: None,
        id: Some(filename.clone())
    };

    

    database::collection("uploads").await.insert_one(doc! {
        "id": filename.clone(),
        "filename": format!("{}.{}", filename, ext),
        "uploader": user.unwrap().get_str("username").unwrap(),
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        "type": info.r#type.clone()
    }, None).await.unwrap();

    HttpResponse::Ok().body(serde_json::to_string(&obj).unwrap())
}