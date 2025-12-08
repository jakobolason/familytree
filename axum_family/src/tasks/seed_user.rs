use loco_rs::{hash, prelude::*};

use crate::models::_entities::user;

#[allow(clippy::module_name_repetitions)]
pub struct SeedUser;
#[async_trait]
impl Task for SeedUser {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_user".to_string(),
            detail: "Task for creating a test user".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let username = vars.cli_arg("username")?;

        let passwd = vars.cli_arg("passwd")?;
        // now create users in db
        let exists = user::Entity::find()
            .filter(user::Column::Email.eq(username))
            .one(&app_context.db)
            .await?;

        if exists.is_some() {
            println!("user already exists: {}", username);
            std::process::exit(1);
        }
        // Important to remember how this looks
        let hashed_password = hash::hash_password(passwd)
            .map_err(|e| Error::Message(format!("Password hashing error: {}", e)))?;

        let user = user::ActiveModel {
            pid: Set(Uuid::new_v4()),
            name: Set(username.clone()),
            email: Set(username.clone()),
            password: Set(hashed_password),
            api_key: Set(format!("key-{}", Uuid::new_v4())),
            ..Default::default()
        };
        match user.insert(&app_context.db).await {
            Ok(_) => println!("Created user {} (pass: {})", username, passwd),
            Err(e) => eprintln!("Error creating user {}: {}", username, e),
        }

        Ok(())
    }
}
