//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Games")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    #[sea_orm(column_type = "Text")]
    pub name: String,
    #[sea_orm(column_type = "Text")]
    pub desc: String,
    #[sea_orm(column_type = "Text")]
    pub author: String,
    #[sea_orm(column_type = "Text")]
    pub genre: String,
    #[sea_orm(column_type = "Text")]
    pub about: String,
    #[sea_orm(column_type = "Text")]
    pub avatar: String,
    #[sea_orm(column_type = "Text")]
    pub trailer: String,
    #[sea_orm(column_type = "Text")]
    pub file: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub screenshots: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_library::Entity")]
    UserLibrary,
}

impl Related<super::user_library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserLibrary.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
