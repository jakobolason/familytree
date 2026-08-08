pub use super::_entities::medlem::{ActiveModel, Entity, Model};
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
pub type Medlem = Entity;

use crate::models::{_entities::medlem, medlem_editors, user};

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
    /// Finds a medlem by provided name
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

    /// Finds a medlem by provided pid
    ///
    /// # Errors
    ///
    /// When could not find medlem by the given token or DB query error
    pub async fn find_by_pid(db: &DatabaseConnection, pid: Uuid) -> ModelResult<Self> {
        let medlem = medlem::Entity::find()
            .filter(
                model::query::condition()
                    .eq(medlem::Column::Pid, pid)
                    .build(),
            )
            .one(db)
            .await?;
        medlem.ok_or_else(|| ModelError::EntityNotFound)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeableFields {
    pub email: Option<String>,
    pub phone_nr: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
}

// implement your write-oriented logic here
impl ActiveModel {
    /// Assumed that who has writing permissions is checked before calling this function
    pub async fn change_fields(
        mut self,
        db: &DatabaseConnection,
        req_pid: &str,
        fields: ChangeableFields,
    ) -> ModelResult<Model> {
        let req_pid: Uuid = user::Model::find_by_pid(db, req_pid).await?.pid;
        let is_owner = self.pid == Set(req_pid);
        let is_editor =
            medlem_editors::Model::is_user_editor(db, &self.pid.clone().unwrap(), &req_pid).await?;

        if !is_owner && !is_editor {
            return Err(ModelError::DbErr(DbErr::Custom("Unauthorized".to_owned())));
        }

        if let Some(email) = fields.email {
            self.email = Set(email);
        }
        if let Some(phone_nr) = fields.phone_nr {
            self.phone_nr = Set(Some(phone_nr));
        }
        if let Some(address) = fields.address {
            self.address = Set(Some(address));
        }
        if let Some(city) = fields.city {
            self.city = Set(Some(city));
        }

        self.update(db).await.map_err(ModelError::from)
    }
}

// implement your custom finders, selectors oriented logic here
impl Entity {}
