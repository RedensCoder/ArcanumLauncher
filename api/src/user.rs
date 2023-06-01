use axum::Json;
use axum::extract::State;
use dotenvy::dotenv;

use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection, QueryFilter, ColumnTrait, EntityTrait};

use crate::{entities::users, security::{AuthReg, create_token}, security};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String
}
impl User {
    pub fn to_userauth(&self) -> UserAuth {
        let user_auth = UserAuth{username: self.username.clone(), password: self.password.clone()};
        return user_auth; 
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAuth {
    pub username: String,
    pub password: String
}

//State(db): State<DatabaseConnection> это нужно чтобы принять State из bind
pub(crate) async fn registration( State(db): State<DatabaseConnection>, Json(body): Json<AuthReg>) -> String {
    let user:&User = &body.reg().unwrap().clone();
    let username = md5::compute(user.username.clone());
    let password = md5::compute(user.password.clone());
    let email = md5::compute(user.email.clone());

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
                username: Set(format!("{:?}",username)),
                password: Set(format!("{:?}",password)),
                email: Set(format!("{:?}",email)),
                about: Set(Some("".to_string())),
                avatar: Set("http://127.0.0.1:8080/api/v1/img/avatar_none.png".to_string()),
                lvl: Set(0),
                ..Default::default()
            };
            
            new_user.insert(&db).await.unwrap();
            dotenv().ok();
            // let token = encode(&Header::default(), &(body.username.clone(), body.password.clone()), &EncodingKey::from_secret(dotenv!("SECRET").as_ref())).unwrap();
            let token = create_token(&body.clone()).unwrap();
            return token;
           
        }
    }
}
// auth 
pub(crate) async fn auth( State(db): State<DatabaseConnection>,Json(body): Json<AuthReg>) -> String {
    let user:&UserAuth = &body.auth().unwrap().clone();
    let username = md5::compute(user.username.clone());
    let password = md5::compute(user.password.clone());

    let user = users::Entity::find()
    .filter(users::Column::Username.eq(format!("{:?}", username)))
    .filter(users::Column::Password.eq(format!("{:?}", password)))
    .one(&db)
    .await.unwrap();

    match user {
        Some(_) => {
            dotenv().ok();
            let token = create_token(&body.clone()).unwrap();
            return String::from(token);
        },
        None => {
            return String::from("User not found!");
        }
    }
    
}