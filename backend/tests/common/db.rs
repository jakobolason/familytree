use loco_rs::prelude::{ColumnTrait, EntityTrait, QueryFilter};
use std::collections::BTreeMap;

use familyserver_backend::app::App;
use familyserver_backend::models::_entities::user;
use familyserver_backend::tasks::seed_user::SeedUser;
use loco_rs::{task::Task, testing::prelude::*};

#[tokio::test]
async fn test_create_user_task() {
    let boot = boot_test::<App>().await.unwrap();

    let vars = loco_rs::task::Vars {
        cli: BTreeMap::from([
            ("username".to_string(), "test_admin@example.com".to_string()),
            ("passwd".to_string(), "password123".to_string()),
        ]),
    };

    let task = SeedUser;
    task.run(&boot.app_context, &vars)
        .await
        .expect("The SeedUser task returned an error");
    let db_user = user::Entity::find()
        .filter(user::Column::Email.eq("test_admin@example.com"))
        .one(&boot.app_context.db)
        .await
        .expect("Failed to execute database query");

    assert!(
        db_user.is_some(),
        "The task ran, but the user was not found in the database!"
    );
}
