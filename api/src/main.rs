#![allow(unused_variables)]
use std::{path::Path, fs};
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, ConnectionTrait, Statement};
use axum::{Router, routing::post};
use dotenvy_macro::dotenv;
use tower_http::cors::{Any, CorsLayer};

mod entities;
mod security;
mod user;


const API_URL: &str = "/api/v1";


#[tokio::main]
async fn main(){
    dotenv().ok();
    let db: DatabaseConnection = Database::connect(dotenv!("DATABASE_URL")).await.unwrap();

    //INITTEST
    let init_sql = fs::read_to_string("database/init.sql").unwrap();
    db.execute(Statement::from_string(sea_orm::DatabaseBackend::Postgres, init_sql)).await.unwrap();
    
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route(format!("{}/reg", API_URL).as_str(), post(user::registration))
        .route(format!("{}/auth", API_URL).as_str(), post( user::auth))
        .layer(cors)
        .with_state(db);
        
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
