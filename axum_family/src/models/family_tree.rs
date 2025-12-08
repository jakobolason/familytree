pub use super::_entities::family_tree::{ActiveModel, Column, Entity, Model};
use loco_rs::prelude::Set;
use once_cell::sync::Lazy;
use sea_orm::entity::prelude::*;
use sea_orm::QueryOrder;
use serde_json::Value;
use std::sync::{Arc, RwLock};

pub type FamilyTree = Entity;

// Global cache:
pub static FAMILY_TREE_CACHE: Lazy<Arc<RwLock<Option<Value>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

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
        Ok(this)
    }
}

// implement your read-oriented logic here
impl Model {
    pub fn get_data(&self) -> Option<&Value> {
        self.tree_data.as_ref()
    }
}

// implement your write-oriented logic here
impl ActiveModel {
    pub async fn create_snapshot(
        db: &DatabaseConnection,
        json_data: Value,
    ) -> Result<Model, DbErr> {
        let entry = ActiveModel {
            tree_data: Set(Some(json_data.clone())),
            ..Default::default()
        };

        let saved: Model = entry.insert(db).await?;

        if let Ok(mut cache) = FAMILY_TREE_CACHE.write() {
            *cache = Some(json_data);
            tracing::info!("Family tree cache updated in memory");
        }
        Ok(saved)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {
    pub async fn retrieve_latest(db: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
        Entity::find()
            .order_by_desc(Column::UpdatedAt)
            .one(db)
            .await
    }

    pub async fn load_cache_from_db(db: &DatabaseConnection) -> Result<(), DbErr> {
        let latest = Self::retrieve_latest(db).await?;

        if let Some(model) = latest {
            if let Some(data) = model.tree_data {
                if let Ok(mut cache) = FAMILY_TREE_CACHE.write() {
                    *cache = Some(data);
                    tracing::info!("Family tree loaded from db into cache")
                }
            }
        } else {
            tracing::warn!("No failmy tree data found in db to cache");
        }
        Ok(())
    }
}
