use axum::{extract::State, Json};
use sea_orm::{DatabaseConnection, ActiveModelTrait, EntityTrait, IntoActiveModel, Set, ModelTrait};
use serde::{Deserialize, Serialize};

use crate::{entities::users::{self, Entity}, user::UserAuth};
#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum Atributs {
    #[serde(rename = "password")]
    Password(String),
    #[serde(rename = "email")]
    Email(String),
    #[serde(rename = "about")]
    About(String),
    #[serde(rename = "lvl")]
    Lvl(i32),
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct UpdateRequest {
    pub username: String,
    pub password: String,
    pub atribut: Atributs,
}

impl From<Atributs> for UpdateRequest {
    fn from(atribut: Atributs) -> Self {
        let username = String::new();
        let password = String::new();
        let mut result = Self {
            username,
            password,
            atribut : atribut.clone(),
        };
        match atribut {
            Atributs::Password(password_value) => {
                result.password = password_value;
            },
            Atributs::Email(email_value) => {
                result.atribut = Atributs::Email(email_value);
            },
            Atributs::About(about_value) => {
                result.atribut = Atributs::About(about_value);
            },
            Atributs::Lvl(lvl_value) => {
                result.atribut = Atributs::Lvl(lvl_value);
            }
        }
        result
    }
}
// это модель запроса в боди джсон для функции смены атрибутов update()
// {
//     "username": "adriann",
//     "password": "pricol",
//     "atribut": {
//         "password": "9999" //здесь password сам по себе указывает тип атрибута поэтому не нужно дописать еще и поле с типом как в функции auth или рег
//     }
// }
pub async fn update(State(db): State<DatabaseConnection>, Json(body): Json<UpdateRequest>){
    let mut user = users::Entity::find_by_id(&body.username)
        .one(&db)
        .await
        .unwrap();
    let mut user: users::ActiveModel = user.unwrap().into();
    match &body.atribut {
        Atributs::Password(password) => {
            user.password = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Unchanged(password).unwrap().to_string());
            println!("{:?}", &user);
        }
        Atributs::Email(email) => {
            user.email = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Unchanged(email).unwrap().to_string());
        }
        Atributs::About(about) => {
            user.about = sea_orm::ActiveValue::Set(Some(sea_orm::ActiveValue::Unchanged(about).unwrap().to_string()));
        }
        Atributs::Lvl(lvl) => {
            user.lvl = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Set(*lvl as i32 ).unwrap());
        }
    }
    
    user.save(&db).await.unwrap();

}
pub async fn delete_by_auth_json(State(db): State<DatabaseConnection>, Json(body): Json<UserAuth>){
    let mut user = Entity::find_by_id(body.username)
    .one(&db)
    .await.unwrap();
    
    user.unwrap().delete(&db).await.unwrap();
    
    println!("мы нашли его спустя полгода и удалили");
}

// pub async fn update_username(State(db): State<DatabaseConnection>){
//     let user = users::Entity::find_by_id("8943ac345b34277db00533dc20f1fb1c")
//     .one(&db.clone())
//     .await.unwrap();

// let mut user: users::ActiveModel = user.unwrap().into();

// user.username = Set();
// user.update(&db).await.unwrap();
// }
