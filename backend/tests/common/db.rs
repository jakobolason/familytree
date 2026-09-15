use std::collections::BTreeMap;

use familyserver_backend::app::App;
use familyserver_backend::tasks::seed_user::SeedUser;
use loco_rs::{task::Task, task::Vars, testing::prelude::*}; // (Adjust this to your actual task name)

#[tokio::test]
async fn test_create_user_task() {
    let boot = boot_test::<App>().await.unwrap();

    let vars = loco_rs::task::Vars {
        cli: BTreeMap::from([
            ("username".to_string(), "test_admin".to_string()),
            ("passwd".to_string(), "password123".to_string()), // Fixed typo: passwrd -> passwd
        ]),
    };

    let task = SeedUser; // Instantiate your task struct
    let result = task
        .run(&boot.app_context, &vars)
        .await
        .expect("The SeedUser task returned an error");
    //. (Optional) Query the DB using boot.app_context.db to verify the user was created
}
