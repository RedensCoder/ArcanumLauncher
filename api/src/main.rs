#![allow(unused_variables)]
use std::{fs};
use bytes::avatars_bytes;
use crud::{update, delete_by_auth_json, add_purcesh, add_playtime};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement};
use axum::{Router, routing::{post, get}};
use dotenvy_macro::dotenv;
use tower_http::cors::{Any, CorsLayer};
use user::{registration, auth, get_user_by_username};

mod entities;
mod security;
mod user;
mod bytes;
mod crud;

//андрюх если ты смотришь то я сделал ток базуданных и вот эту sea-orm-cli а остольное тоже самое, я уже в четверг буду заниматся остальном 
const API_URL: &str = "/api/v1";
fn route(route:&str)-> String{
   return format!("{}/{}", API_URL, route)
}
#[tokio::main]
async fn main(){
    
    // dotenv используется для того чтобы сохранять то что не должно быть увиденым чужими глазами это пOроли и так далее
    dotenv().ok();
    // здесь идет подключения к дата бэйс постгрес 
    let db: DatabaseConnection = Database::connect(dotenv!("DATABASE_URL")).await.unwrap();
    
    //INIT
    let users_sql = fs::read_to_string("database/users.sql").unwrap();
    db.execute(Statement::from_string(sea_orm::DatabaseBackend::Postgres, users_sql)).await.unwrap();
    let purchase_sql = fs::read_to_string("database/purchase.sql").unwrap();
    db.execute(Statement::from_string(sea_orm::DatabaseBackend::Postgres, purchase_sql)).await.unwrap();
    let games_sql = fs::read_to_string("database/games.sql").unwrap();
    db.execute(Statement::from_string(sea_orm::DatabaseBackend::Postgres, games_sql)).await.unwrap();

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route(&route("reg"), post(registration))
        .route(&route("auth"), post(auth))
        .route(&route("img/:name"), get(avatars_bytes))
        .route(&route("upload"), post(bytes::upload_file))
        .route(&route("getUserByUsername/:username"), get(get_user_by_username))
        .route(&route("deleteByAuthJson"), post(delete_by_auth_json))
        .route(&route("updateAtribut"), post(update))
        .route(&route("addPurchase"), post(add_purcesh))
        .route(&route("addPlayTime"), post(add_playtime))
        //здесь используется with_state который позволяет передать переменную всем маршрутом (это самая важная чась это твари заняла у меня 2 дня)
        .with_state(db)
        .layer(cors);
       
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
