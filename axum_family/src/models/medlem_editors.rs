use sea_orm::entity::prelude::*;

pub use super::_entities::medlem_editors::{ActiveModel, Column, Entity, Model};
pub type MedlemEditors = Entity;
use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::model::query::condition,
};
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, _insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

// implement your read-oriented logic here
impl Model {
    pub async fn is_user_editor(
        db: &DatabaseConnection,
        medlem_pid: &Uuid,
        user_pid: &Uuid,
    ) -> ModelResult<bool> {
        let count = Entity::find()
            .filter(condition().eq(Column::MedlemPid, *medlem_pid))
            .filter(condition().eq(Column::UserPid, *user_pid))
            .filter(condition().eq(Column::Role, "editor"))
            .count(db)
            .await?;
        Ok(count > 0)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
