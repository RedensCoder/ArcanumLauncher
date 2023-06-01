#![allow(unused_variables)]
use std::{fs};
use bytes::avatars_bytes;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement};
use axum::{Router, routing::{post, get}};
use dotenvy_macro::dotenv;
use tower_http::cors::{Any, CorsLayer};
use user::{registration, auth};

mod entities;
mod security;
mod user;
mod bytes;

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
    let library_sql = fs::read_to_string("database/user_library.sql").unwrap();
    db.execute(Statement::from_string(sea_orm::DatabaseBackend::Postgres, library_sql)).await.unwrap();
    
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route(&route("reg"), post(registration))
        .route(&route("auth"), post(auth))
        .route(&route("img/:name"), get(avatars_bytes))
        .layer(cors)
        //здесь используется with_state который позволяет передать переменную всем маршрутом (это самая важная чась это твари заняла у меня 2 дня)
        .with_state(db);
        
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
