use axum::{extract::{Path, State}, Json};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::{Serialize, Deserialize};

use crate::entities::games;

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub gamename: String,
    pub desc: String,
    pub author: String,
    pub genre: String,
    pub about: String,
    pub avatar: String,
    pub trailer: String,
    pub screenshots: String,
    pub file: String,
}

pub async fn get_game_by_id(State(db): State<DatabaseConnection>, Path(id): Path<String>) -> Json<Option<serde_json::Value>> {
    let game = games::Entity::find()
        .filter(games::Column::Gamename.eq(id))
        .one(&db)
        .await.unwrap();

    match game {
        Some(_) => {
            let game: games::ActiveModel = game.unwrap().into();

            let new_game: Game = Game {
                gamename: game.gamename.unwrap(),
                desc: game.desc.unwrap().unwrap(),
                author: game.author.unwrap().unwrap(),
                genre: game.genre.unwrap().unwrap(),
                about: game.about.unwrap().unwrap(),
                avatar: game.avatar.unwrap().unwrap(),
                trailer: game.trailer.unwrap().unwrap(),
                screenshots: game.screenshots.unwrap().unwrap(),
                file: game.file.unwrap().unwrap()
            };

            return Json(Some(serde_json::to_value(new_game).unwrap()));
        },
        None => Json(None)
    } 
}

pub async fn get_all(State(db): State<DatabaseConnection>) -> Json<Vec<Game>> {
    let games = games::Entity::find()
        .all(&db)
        .await.unwrap();

    let mut game_arr: Vec<Game> = vec![];

    for el in games {
        let game: games::ActiveModel = el.into();

            let new_game: Game = Game {
                gamename: game.gamename.unwrap(),
                desc: game.desc.unwrap().unwrap(),
                author: game.author.unwrap().unwrap(),
                genre: game.genre.unwrap().unwrap(),
                about: game.about.unwrap().unwrap(),
                avatar: game.avatar.unwrap().unwrap(),
                trailer: game.trailer.unwrap().unwrap(),
                screenshots: game.screenshots.unwrap().unwrap(),
                file: game.file.unwrap().unwrap()
            };

            game_arr.push(new_game);
    }

    return Json(game_arr);
}