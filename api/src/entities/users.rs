//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub about: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub avatar: String,
    pub lvl: i32,
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
