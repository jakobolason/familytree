pub use super::_entities::medlem::{ActiveModel, Entity, Model};
use sea_orm::entity::prelude::*;
pub type Medlem = Entity;

use crate::models::_entities::medlem;

use loco_rs::{
    model::{ModelError, ModelResult},
    prelude::model,
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
    /// Finds a medlem by provided email
    ///
    /// # Errors
    ///
    /// When could not find medlem by the given token or DB query error
    pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> ModelResult<Self> {
        let medlem = medlem::Entity::find()
            .filter(
                model::query::condition()
                    .eq(medlem::Column::Name, name)
                    .build(),
            )
            .one(db)
            .await?;
        medlem.ok_or_else(|| ModelError::EntityNotFound)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
