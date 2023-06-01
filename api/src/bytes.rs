use std::fs;

use axum::{extract::Path, body::{Full, Bytes}, response::Response};

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