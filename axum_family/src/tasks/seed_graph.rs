use loco_rs::{hash, prelude::*};
use std::path::Path;

use crate::{
    grapher::graph_creater::{run_grapher, D3Node, Person},
    models::_entities::user,
    models::family_tree::{ActiveModel as FamilyTreeActive, FamilyTree},
};

fn collect_people(node: &D3Node) -> Vec<Person> {
    let mut people = vec![node.person.clone()];

    for child in &node.children {
        people.extend(collect_people(child));
    }
    people
}

#[allow(clippy::module_name_repetitions)]
pub struct SeedTree;
#[async_trait]
impl Task for SeedTree {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "seed_tree".to_string(),
            detail: "Task for creating family tree from .xls file".to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        println!("Family Tree D3 Export Task...");
        let path = vars.cli_arg("path");

        let default_path = "family_data.xls";
        let file_path = match path {
            Ok(path) => path,
            Err(_) => {
                println!("No file specified, defaulting to '{}'", default_path);
                default_path
            }
        };

        let path = Path::new(file_path);

        if !path.exists() {
            eprintln!("Error: File '{}' not found.", file_path);
            std::process::exit(1);
        }

        let tree_nodes = match run_grapher(path) {
            Ok(nodes) => {
                println!("Task Complete!");
                println!("   File 'family_data.js' has been created.");
                nodes
            }
            Err(e) => {
                eprintln!("Error generating tree: {}", e);
                std::process::exit(1);
            }
        };
        let json_value = serde_json::to_value(&tree_nodes)?;
        FamilyTreeActive::create_snapshot(&app_context.db, json_value)
            .await
            .map_err(|e| Error::Message(format!("JSON serialization error: {}", e)))?;

        // now create users in db
        let all_people = collect_people(&tree_nodes[0]);
        for person in all_people {
            // I need all of these, otherwise the person might not be interested either way
            if person.email.trim().is_empty()
                || person.name.trim().is_empty()
                || person.mobile_number.trim().is_empty()
            {
                continue;
            }
            let exists = user::Entity::find()
                .filter(user::Column::Email.eq(&person.email))
                .one(&app_context.db)
                .await?;

            if exists.is_some() {
                println!("user already exists: {}", person.email);
                continue;
            }
            // Important to remember how this looks
            let hashed_password = hash::hash_password(person.mobile_number.trim())
                .map_err(|e| Error::Message(format!("Password hashing error: {}", e)))?;

            let user = user::ActiveModel {
                pid: Set(Uuid::new_v4()),
                name: Set(person.name.clone()),
                email: Set(person.email.clone()),
                password: Set(hashed_password),
                api_key: Set(format!("key-{}", Uuid::new_v4())),
                ..Default::default()
            };
            match user.insert(&app_context.db).await {
                Ok(_) => println!(
                    "Created user {} (pass: {})",
                    person.email, person.mobile_number
                ),
                Err(e) => eprintln!("Error creating user {}: {}", person.email, e),
            }
        }
        println!("Tree generated successfully!");

        Ok(())
    }
}
