
use axum::{extract::State, Json, TypedHeader, headers::{Authorization, authorization::Bearer}};
use sea_orm::{DatabaseConnection, ActiveModelTrait, EntityTrait, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};

use crate::{entities::{users::{self}, purchase::{self}, games}, security::verify};
//////////////////////////////////////////////CRUD FOR users.rs
#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum Atributs {
    #[serde(rename = "nickname")]
    Nickname(String),
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
            Atributs::Nickname(nickname_value) => {
                result.atribut = Atributs::Nickname(nickname_value);
            },
             Atributs::Password(password_value) => {
                result.atribut = Atributs::Password(password_value);
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
pub async fn update(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(db): State<DatabaseConnection>, Json(body): Json<UpdateRequest>){
    match verify(auth.token(), &db).await {
        Some(_) => {
            let user = users::Entity::find_by_id(&body.username)
                .one(&db)
                .await
                .unwrap();
            let mut user: users::ActiveModel = user.unwrap().into();
            match &body.atribut {
                Atributs::Nickname(nickname) => {
                    user.nickname = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Unchanged(nickname).unwrap().to_string());
                }
                Atributs::Password(password) => {
                    user.password = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Unchanged(password).unwrap().to_string());
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
            
            user.update(&db).await.unwrap();
        },
        None => {}
    }
    
    
}
// pub async fn delete_by_auth_json(State(db): State<DatabaseConnection>, Json(body): Json<UserAuth>){
//     let user = Entity::find_by_id(body.username)
//     .one(&db)
//     .await.unwrap();
    
//     user.unwrap().delete(&db).await.unwrap();
    
//     println!("мы нашли его спустя полгода и удалили");
// }
////////////////////////////////////////////////////CRUD FOR purchase.rs
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Purchase{
    pub username: String,
    pub password: String,
    pub gamename: String,
}
#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct UserTime{
    pub username: String,
    pub gamename: String,
    pub playtime: f32
}

/////пример запроса /addPurchase
// {
//   "username":"adrian",
//   "password":"popricol",
//   "gamename":"naruto"
// }
pub async fn add_purcesh(TypedHeader(auth): TypedHeader<Authorization<Bearer>>, State(db): State<DatabaseConnection>, Json(body): Json<Purchase>)-> String{
    match verify(auth.token(), &db).await {
        Some(_) => {
            let user = users::Entity::find_by_id(body.username.clone())
                .one(&db)
                .await.unwrap();

            let game = games::Entity::find_by_id(body.gamename)
                .one(&db)
                .await.unwrap();

            let purchases = purchase::Entity::find()
                .filter(purchase::Column::Username.eq(body.username))
                .one(&db)
                .await.unwrap();

            match user {
                Some(user) => {
                        match game {
                            Some(game) => {
                                match purchases {
                                    Some(_) => {
                                        return String::from("Такая игра уже приобретена!");
                                    },
                                    None => {
                                        let new_purchase = purchase::ActiveModel {
                                            username:Set(user.username),
                                            game:Set(game.gamename),
                                            ..Default::default()
                                        };
                                        new_purchase.insert(&db.clone()).await.unwrap();
                                        return String::from("Вы успешно совершили покупку!");
                                    }
                                }
                            },
                            None => {
                                return String::from("Возможно возникла ошибка, пока мы не владеем такой игрой попробуйте с ново");
                            }
                        }
                },
                None => {
                    return String::from("Возможно возникла ошибка, пользователь небыл найден попробуйте с ново");
            }
            }
        },
        None => String::from("Null")
    }
    

}
//////счетчик часов в игре
/// пример запроса для /addPlayTime
// {
//     "username":"adrian",
//     "gamename":"naruto",
//     "playtime": 1.25
//   }
pub async fn add_playtime(State(db): State<DatabaseConnection>, Json(body): Json<UserTime>)-> String{
    let entries = purchase::Entity::find()
        .filter(purchase::Column::Username.eq(body.username))
        .filter(purchase::Column::Game.eq( body.gamename))
        .one(&db)
        .await.unwrap();

    match entries {
        Some(entires) => {
            let mut conv : purchase::ActiveModel = entires.clone().into(); 
            conv.hours = sea_orm::ActiveValue::Set(sea_orm::ActiveValue::Unchanged(entires.clone().hours).unwrap()+ body.playtime);
            conv.update(&db).await.unwrap();
            String::from("Наигранное время успешно было внесена в базу данных")

        },
        None => {String::from("Возможно возникла ошибка, пользователь небыл найден попробуйте с ново")},
    }
}