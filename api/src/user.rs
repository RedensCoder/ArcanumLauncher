use axum::Json;
use axum::extract::State;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, Set, Database,  DatabaseConnection, QueryFilter, ColumnTrait, EntityTrait, ConnectionTrait};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::{entities::users};
use std::sync::{Arc, Once, Mutex};
use std::future::Future;
// lazy_static! {
//     pub static ref DB: Mutex<DatabaseConnection> = Mutex::new(Database::connect(dotenv!("DATABASE_URL")).await.unwrap());
// }


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub user: User,
    pub exp: i64
}

fn create_token(user:User) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now() + chrono::Duration::hours(1);
    let my_claims = Claims {
        user,
        exp : exp.timestamp() as i64
    };
    dotenv().ok();
    let key = dotenv!("SECRET");
    
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_ref()))?;
    Ok(token)
}
//State(db): State<DatabaseConnection> это нужно чтобы принять State из bind
pub async fn registration( State(db): State<DatabaseConnection>, Json(body): Json<User>) -> String {
    let username = md5::compute(body.username.clone());
    let password = md5::compute(body.password.clone());

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
                ..Default::default()
            };
            
            new_user.insert(&db).await.unwrap();
            dotenv().ok();
            let token = encode(&Header::default(), &(body.username.clone(), body.password.clone()), &EncodingKey::from_secret(dotenv!("SECRET").as_ref())).unwrap();
            return String::from(token);
        }
    }
}
// auth
pub async fn auth( State(db): State<DatabaseConnection>,Json(body): Json<User>) -> String {
    let username = md5::compute(body.username.clone());
    let password = md5::compute(body.password.clone());

    let user = users::Entity::find()
    .filter(users::Column::Username.eq(format!("{:?}", username)))
    .filter(users::Column::Password.eq(format!("{:?}", password)))
    .one(&db)
    .await.unwrap();

    match user {
        Some(_) => {
            dotenv().ok();
            let token = encode(&Header::default(), &(body.username.clone(), body.password.clone()), &EncodingKey::from_secret(dotenv!("SECRET").as_ref())).unwrap();
            return String::from(token);
        },
        None => {
            return String::from("User not found!");
        }
    }
    
}