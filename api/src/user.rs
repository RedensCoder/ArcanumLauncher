use axum::Json;
use axum::extract::State;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection, QueryFilter, ColumnTrait, EntityTrait};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::{entities::users};



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
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum AuthReg {
    #[serde(rename = "reg")]
    User(User),
    #[serde(rename = "auth")]
    UserAuth(UserAuth) 
}

impl AuthReg {
    pub fn to_user_auth(&self) -> UserAuth {
        match self {
            AuthReg::User(user_reg) => UserAuth {
                username: user_reg.username.clone(),
                password: user_reg.password.clone(),
            },
            AuthReg::UserAuth(user_auth) => UserAuth {
                username: user_auth.username.clone(),
                password: user_auth.password.clone(),
            },
        }
    }
    pub fn reg(&self) -> Result<&User, &UserAuth> {
        match &self {
            AuthReg::User(user) => Ok(user),
            AuthReg::UserAuth(user) => Err(user)
        }
    }
    pub fn auth(&self) -> Result<&UserAuth, &User> {
        match &self {
            AuthReg::UserAuth(user) => Ok(user),
            AuthReg::User(user) => Err(user)
        }
    }
}
     

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub user: UserAuth,
    pub exp: i64
}

fn create_token(body:&AuthReg) -> Result<String, String> {
    let exp = chrono::Utc::now() + chrono::Duration::hours(1);
    let user = body.to_user_auth();
    let my_claims = Claims {
        user,
        exp : exp.timestamp() as i64
    };
    dotenv().ok();
    let key = dotenv!("SECRET");
    
    let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_ref())){
        Ok(t) => t,
        Err(e) => return Err(format!("Ошибка при создании токена: {}", e))
    };
    Ok(token)
}
//State(db): State<DatabaseConnection> это нужно чтобы принять State из bind
pub async fn registration( State(db): State<DatabaseConnection>, Json(body): Json<AuthReg>) -> String {
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
pub async fn auth( State(db): State<DatabaseConnection>,Json(body): Json<AuthReg>) -> String {
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