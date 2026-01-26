use crate::{models::_entities::user, models::family_tree::ActiveModel as FamilyTreeActive};
use family_graph::{
    create_d3_export,
    family_graph::{D3Node, Person},
    run_grapher, CreateOptions,
};
use loco_rs::{hash, prelude::*};
use std::path::Path;

pub const RANDOM_PASSWD_LENGTH: i8 = 20;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FamilyTreeNode {
    name: String,
    children: Vec<FamilyTreeNode>,
}

fn collect_people(node: &D3Node) -> Vec<Person> {
    // TODO: Children should only be the immediate children, not grand etc.
    let mut people = vec![node.person.clone()];

    for child in &node.children {
        people.extend(collect_people(child));
    }
    people
}

impl FamilyTreeNode {
    pub fn new(name: &str) -> Self {
        FamilyTreeNode {
            name: name.to_string(),
            children: Vec::new(),
        }
    }
    fn create_d3_tree(node: &D3Node) -> Self {
        Self {
            name: node.person.name.clone(),
            children: node.children.iter().map(Self::create_d3_tree).collect(),
        }
    }
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

        let family_graph = match run_grapher(path, "Ark1") {
            Ok(graph) => {
                println!("Task Complete!");
                println!("   File 'family_data.js' has been created.");
                graph
            }
            Err(e) => {
                eprintln!("Error generating tree: {}", e);
                std::process::exit(1);
            }
        };

        // TODO: Using this function only to avoid having to include petgraph types, but in
        // the future, i should make the filtering myself
        let tree_nodes = create_d3_export(&family_graph, "family_data.js")
            .expect("Could not create family_data.js file");
        // Strips tree_nodes of information, such that only tree remains
        let only_tree = tree_nodes
            .first()
            .map(FamilyTreeNode::create_d3_tree)
            .expect("The first entry should be the forefather, who is not present");

        let json_value = serde_json::to_value(&only_tree)?;
        // Now save the tree in memory
        FamilyTreeActive::create_snapshot(&app_context.db, json_value)
            .await
            .map_err(|e| Error::Message(format!("JSON serialization error: {}", e)))?;

        // TODO: Now we should ensure there is a user for each person.

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
            // TODO: Should alter this to a random generation
            let random_str = hash::random_string(RANDOM_PASSWD_LENGTH as usize);
            let hashed_password = hash::hash_password(&random_str)
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

        // TODO: Then, 'medlem' should be updated with info from tree_nodes

        println!("Tree generated successfully!");

        Ok(())
    }
}
