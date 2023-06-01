use dotenvy_macro::dotenv;
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::{decode, DecodingKey, Validation, errors::ErrorKind};
use sea_orm::{QueryFilter, ColumnTrait, EntityTrait, DatabaseConnection};
use serde::{Serialize, Deserialize};
use crate::user::{UserAuth, User};
use crate::entities::users;

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

pub(crate) fn create_token(body:&AuthReg) -> Result<String, String> {
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
pub async fn verify(token: &str, key: &str, db: DatabaseConnection) -> Result<(), jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(key.as_ref()), &Validation::default())?;
    let now: i64 = chrono::Utc::now().timestamp();
    if token_data.claims.exp < now {
        return Err(ErrorKind::ExpiredSignature.into());
    }

    let user = users::Entity::find()
    .filter(users::Column::Username.eq(format!("{:?}", token_data.claims.user.username)))
    .filter(users::Column::Password.eq(format!("{:?}", token_data.claims.user.password)))
    .one(&db)
    .await.unwrap();

    Ok(())
}