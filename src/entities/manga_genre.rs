//! SeaORM Entity. Generated by sea-orm-codegen 0.9.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "manga_genre")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub manga_genre_id: i32,
    pub genre_id: String,
    pub manga_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::genre::Entity",
        from = "Column::GenreId",
        to = "super::genre::Column::GenreId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Genre,
    #[sea_orm(
        belongs_to = "super::manga::Entity",
        from = "Column::MangaId",
        to = "super::manga::Column::MangaId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Manga,
}

impl Related<super::genre::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Genre.def()
    }
}

impl Related<super::manga::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Manga.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}