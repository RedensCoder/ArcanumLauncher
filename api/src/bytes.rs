use std::{fs::{self}};

use axum::{extract::{Path, State}, body::{Full, Bytes}, response::Response, headers::{authorization::Bearer, Authorization}, TypedHeader};

use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter, Set, ActiveModelTrait,};
use tokio::{fs::File, io::AsyncWriteExt};
use crate::{security::verify, entities::users};

pub async fn avatars_bytes(Path(name):Path<String>) -> Result<Response<Full<Bytes>>, String>{
   let file = fs::read(format!("data_bytes/users_avatar/{}", name));

   match file {
      Ok(_) => {
        Ok(
        Response::builder()
         .header("Content-Type", "image/png")
         .body(Full::from(file.unwrap()))
         .unwrap()
        )
      },
      error => return Err(String::from("К сожелению аватар с таким именем не найден"))
   }
}

pub async fn game_avatar(Path(name):Path<String>) -> Result<Response<Full<Bytes>>, String>{
    let file = fs::read(format!("data_bytes/games_avatar/{}.png", name));
 
    match file {
       Ok(_) => {
         Ok(
         Response::builder()
          .header("Content-Type", "image/png")
          .body(Full::from(file.unwrap()))
          .unwrap()
         )
       },
       error => return Err(String::from("К сожелению аватар с таким именем не найден"))
    }
 }

 pub async fn game_trailer(Path(name):Path<String>) -> Result<Response<Full<Bytes>>, String>{
    let file = fs::read(format!("data_bytes/games_trailer/{}.mp4", name));
 
    match file {
       Ok(_) => {
         Ok(
         Response::builder()
          .header("Content-Type", "video/mp4")
          .body(Full::from(file.unwrap()))
          .unwrap()
         )
       },
       error => return Err(String::from("К сожелению аватар с таким именем не найден"))
    }
 }

 pub async fn game_screen(Path(name):Path<String>) -> Result<Response<Full<Bytes>>, String>{
    let file = fs::read(format!("data_bytes/games_screenshots/{}.png", name));
 
    match file {
       Ok(_) => {
         Ok(
         Response::builder()
          .header("Content-Type", "image/png")
          .body(Full::from(file.unwrap()))
          .unwrap()
         )
       },
       error => return Err(String::from("К сожелению аватар с таким именем не найден"))
    }
 }

pub async fn upload_file(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(db): State<DatabaseConnection>, mut bytes: Bytes) -> String { 
    match verify(auth.token(), &db).await {
        Some(token) => {
            let mut file = File::create(format!("data_bytes/users_avatar/{}_avatar.png", token.claims.user.username)).await.unwrap();
            file.write_all(&mut bytes).await.unwrap();

            let user = users::Entity::find()
            .filter(users::Column::Username.eq(format!("{:?}",token.claims.user.username)))
            .one(&db)
            .await.unwrap();

            let user = users::Entity::find()
                .filter(users::Column::Username.eq(format!("{}",token.claims.user.username)))
                .one(&db)
                .await.unwrap();

            let mut user: users::ActiveModel = user.unwrap().into();

            user.avatar = Set(format!("http://127.0.0.1:8080/api/v1/img/{}_avatar.png", token.claims.user.username));
            user.update(&db).await.unwrap();

            return String::from("Файл успешно загружен на сервер");
        },
        None => String::from("Err")
    }

     
}