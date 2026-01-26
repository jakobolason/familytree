use crate::{
    models::_entities::user, models::family_tree::ActiveModel as FamilyTreeActive, models::medlem,
};
use chrono::NaiveDate;
use family_graph::{
    create_d3_export,
    family_graph::{D3Node, Person},
    run_grapher,
};
use loco_rs::{hash, prelude::*};
use std::{collections::HashMap, path::Path};

pub const RANDOM_PASSWD_LENGTH: i8 = 20;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FamilyTreeNode {
    name: String,
    pid: Option<Uuid>,
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
            pid: None,
            children: Vec::new(),
        }
    }
    fn create_d3_tree(node: &D3Node, medlem_pids: &HashMap<String, Uuid>) -> Self {
        let full_name = format!("{} {}", node.person.name, node.person.last_name);
        Self {
            name: node.person.name.clone(),
            pid: medlem_pids.get(&full_name).copied(),
            children: node
                .children
                .iter()
                .map(|n| Self::create_d3_tree(n, medlem_pids))
                .collect(),
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

        // now create users in db
        let mut medlem_pids = HashMap::new();
        let all_people = collect_people(&tree_nodes[0]);
        for person in all_people {
            // I need all of these, otherwise the person might not be interested either way
            if person.email.trim().is_empty()
                || person.name.trim().is_empty()
                || person.mobile_number.trim().is_empty()
                || person.birthdate.contains("-")
            // If this last one is present, then this person has sadly passed
            {
                continue;
            }
            let full_name = format!("{} {}", person.name.clone(), person.last_name.clone());
            let exists = user::Model::find_by_full_name(&app_context.db, &full_name).await;

            let user = match exists {
                Ok(user) => {
                    println!("User {full_name} exists");
                    user
                }
                Err(ModelError::EntityNotFound) => {
                    // NOTE: All users password's are initialized as random
                    let random_str = hash::random_string(RANDOM_PASSWD_LENGTH as usize);
                    let hashed_password = hash::hash_password(&random_str)
                        .map_err(|e| Error::Message(format!("Password hashing error: {}", e)))?;

                    let user = user::ActiveModel {
                        pid: Set(Uuid::new_v4()),
                        // TODO: Should remove '*' from names
                        name: Set(format!(
                            "{} {}",
                            person.name.clone(),
                            person.last_name.clone()
                        )),
                        email: Set(person.email.clone()),
                        password: Set(hashed_password),
                        api_key: Set(format!("key-{}", Uuid::new_v4())),
                        ..Default::default()
                    };
                    match user.insert(&app_context.db).await {
                        Ok(user) => {
                            println!(
                                "Created user {} (pass: {})",
                                person.email, person.mobile_number
                            );
                            user
                        }
                        Err(e) => {
                            eprintln!("Error creating user {}: {}", person.email, e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error checking user {}: {}", person.name, e);
                    continue;
                }
            };

            // Now update information in db, if there is a mismatch
            // Using the user instance
            let user_pid = user.pid;
            let exists = medlem::Model::find_by_name(&app_context.db, &full_name).await;
            let model = match exists {
                Ok(model) => model,
                Err(ModelError::EntityNotFound) => {
                    let parsed_date = NaiveDate::parse_from_str(&person.birthdate, "%d.%m.%y");
                    // Create new medlem entry
                    let new_medlem = medlem::ActiveModel {
                        pid: Set(Uuid::new_v4()),
                        user_pid: Set(Some(user_pid)),
                        name: Set(full_name.clone()),
                        email: Set(person.email.clone()),
                        birthdate: Set(parsed_date.ok()),
                        ..Default::default()
                    };
                    match new_medlem.insert(&app_context.db).await {
                        Ok(model) => {
                            println!("Created medlem entry for {}", full_name);
                            model
                        }
                        Err(e) => {
                            eprintln!("Error creating medlem for {}: {}", full_name, e);
                            continue;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error finding medlem for {}: {}", full_name, e);
                    continue;
                }
            };
            medlem_pids.insert(full_name, model.pid);
        }
        // now

        // Strips tree_nodes of information, such that only tree remains
        let only_tree = tree_nodes
            .first()
            .map(|n| FamilyTreeNode::create_d3_tree(n, &medlem_pids))
            .expect("The first entry should be the forefather, who is not present");

        let json_value = serde_json::to_value(&only_tree)?;
        // Now save the tree in memory
        FamilyTreeActive::create_snapshot(&app_context.db, json_value)
            .await
            .map_err(|e| Error::Message(format!("JSON serialization error: {}", e)))?;

        println!("Tree generated successfully!");

        Ok(())
    }
}
