use axum::Json;
use axum::{headers::{authorization::Bearer, Authorization}, TypedHeader};
use axum::extract::{Path, State};
use dotenvy::dotenv;

use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection, QueryFilter, ColumnTrait, EntityTrait};

use crate::{entities::users, security::{AuthReg, create_token, verify}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullUser {
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub about: String,
    pub avatar: String,
    pub lvl: i32
}

// impl User {
//     pub fn to_userauth(&self) -> UserAuth {
//         let user_auth = UserAuth{username: self.username.clone(), password: self.password.clone()};
//         return user_auth; 
//     }
// }
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

//State(db): State<DatabaseConnection> это нужно чтобы принять State из bind
///////пример запроса для регистрации
//{
//     "type" : "reg",
//     "username": "adrian",
//     "password": "popricolu",
//     "email":"popricolu@.mail"
//   }
pub(crate) async fn registration( State(db): State<DatabaseConnection>, Json(body): Json<AuthReg>) -> String {
    let user:&User = &body.reg().unwrap().clone();
    let username = user.username.clone();
    let password = md5::compute(user.password.clone());
    let email = user.email.clone();

    let user = users::Entity::find()
    .filter(users::Column::Username.eq(format!("{:?}",username)))
    .one(&db)
    .await.unwrap();

    match user {
        Some(_) => {
            return String::from("User is already!");
        },
        None => {
            let new_user = users::ActiveModel {
                username: Set(format!("{}",username.clone())),
                password: Set(format!("{:?}",password.to_owned())),
                email: Set(format!("{}",email)),
                about: Set(Some("".to_string())),
                avatar: Set("http://127.0.0.1:8080/api/v1/img/none_avatar.png".to_string()),
                nickname: Set(format!("{}",username.clone())),
                lvl: Set(1),
                ..Default::default()
            };
            
            let token = create_token(&body.clone(),&new_user.lvl.clone().unwrap()).await.unwrap();
            new_user.insert(&db.clone()).await.unwrap();
            dotenv().ok();
            return token;
        }
    }
}
// auth 
///////пример запроса для авторизации
//{
//     "type" : "auth",
//     "username": "adrian",
//     "password": "popricolu"
//   }
pub(crate) async fn auth( State(db): State<DatabaseConnection>,Json(body): Json<AuthReg>) -> String {
    let user:&UserAuth = &body.auth().unwrap().clone();
    let username = user.username.clone();
    let password = md5::compute(user.password.clone());

    let user = users::Entity::find()
    .filter(users::Column::Username.eq(format!("{}", username)))
    .filter(users::Column::Password.eq(format!("{:?}", password)))
    .one(&db)
    .await.unwrap();

    match user {
        Some(user) => {
            dotenv().ok();
            let token = create_token(&body.clone(), &user.lvl).await.unwrap();
            return String::from(token);
        },
        None => {
            return String::from("User not found!");
        }
    }
    
}

pub async fn get_user_by_username(State(db): State<DatabaseConnection>, Path(username): Path<String>) -> Json<Option<serde_json::Value>> {
        let user = users::Entity::find()
                .filter(users::Column::Username.eq(username))
                .one(&db)
                .await.unwrap();

            let user: users::ActiveModel = user.unwrap().into();

            let new_user: FullUser = FullUser { username: user.username.unwrap(), nickname: user.nickname.unwrap(), password: user.password.unwrap(), email: user.email.unwrap(), about: user.about.unwrap().unwrap(), avatar: user.avatar.unwrap(), lvl: user.lvl.unwrap() };

            return Json(Some(serde_json::to_value(new_user).unwrap()));
}