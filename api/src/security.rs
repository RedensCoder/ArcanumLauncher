use jsonwebtoken::{decode, DecodingKey, Validation, errors::ErrorKind};
use sea_orm::{QueryFilter, ColumnTrait, EntityTrait, DatabaseConnection};
use crate::user::Claims;
use crate::entities::users;

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