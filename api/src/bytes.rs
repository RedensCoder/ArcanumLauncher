use std::{fs::{self}, convert::Infallible};

use axum::{extract::{Path, State}, body::{Full, Bytes}, response::Response, Json, headers::{authorization::Bearer, Authorization}, TypedHeader};
use jsonwebtoken::errors::ErrorKind;
use sea_orm::DatabaseConnection;
use tokio::{fs::File, io::AsyncWriteExt};
use crate::security::verify;

pub async fn avatars_bytes(Path(name):Path<String>) -> Result<Response<Full<Bytes>>, String>{
   let file = fs::read(format!("data_bytes/users_avatar/{}_avatar.png", name));

   match file {
      Ok(_) => {
        Ok(
        Response::builder()
         .header("Content-Type", "image/png")
         .body(Full::from(file.unwrap()))
         .unwrap()
        )
      },
      Error => return Err(String::from("К сожелению аватар с таким именем не найден"))
   }
}
pub async fn upload_file(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(db): State<DatabaseConnection>, mut bytes: Bytes) -> String { 
    
    
    match verify(auth.token(), db).await {
        Some(token) => {
            let mut file = File::create(format!("data_bytes/users_avatar/{}_avatar.png", token.claims.user.username)).await.unwrap();
            file.write_all(&mut bytes).await.unwrap();
            String::from("Файл успешно загружен на сервер")
        },
        None => String::from("Err")
    }

     
}