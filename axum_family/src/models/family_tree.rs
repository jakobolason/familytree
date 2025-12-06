pub use super::_entities::family_tree::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type FamilyTree = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let mut this = self;
        if !insert {
            this.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        }
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
