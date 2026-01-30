use crate::models::{
    _entities::user, family_tree::ActiveModel as FamilyTreeActive, medlem, medlem_editors,
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

impl FamilyTreeNode {
    fn create_d3_tree(node: &D3Node, medlem_pids: &HashMap<String, Uuid>) -> Self {
        let full_name = format!(
            "{} {}",
            node.person.name.clone().trim_start_matches('*').trim(),
            node.person.last_name.clone()
        );
        Self {
            name: full_name.clone(),
            pid: medlem_pids.get(&full_name).copied(),
            children: node
                .children
                .iter()
                .map(|n| Self::create_d3_tree(n, medlem_pids))
                .collect(),
        }
    }
}

fn collect_people(node: &D3Node) -> Vec<Person> {
    // TODO: Children should only be the immediate children, not grand etc.
    let mut people = vec![node.person.clone()];

    for child in &node.children {
        people.extend(collect_people(child));
    }
    people
}

async fn create_user(
    person: &Person,
    full_name: &str,
    db: &DatabaseConnection,
) -> Result<Option<Uuid>, Error> {
    // I need all of these, otherwise the person might not be interested either way
    if person.email.trim().is_empty()
        || person.name.trim().is_empty()
        || person.mobile_number.trim().is_empty()
        || person.birthdate.contains("-")
    // If this last one is present, then this person has sadly passed
    {
        Ok(None)
    } else {
        let exists = user::Model::find_by_full_name(db, full_name).await;

        let user = match exists {
            Ok(user) => {
                println!("User {full_name} exists");
                user
            }
            Err(ModelError::EntityNotFound) => {
                // NOTE: All users password's are initialized as random
                // TODO: This should take the time, as all passwd are the same at start
                let random_str = hash::random_string(RANDOM_PASSWD_LENGTH as usize);
                let hashed_password = hash::hash_password(&random_str)
                    .map_err(|e| Error::Message(format!("Password hashing error: {}", e)))?;

                let user = user::ActiveModel {
                    pid: Set(Uuid::new_v4()),
                    // TODO: Should remove '*' from names
                    name: Set(full_name.to_string()),
                    email: Set(person.email.clone()),
                    password: Set(hashed_password),
                    api_key: Set(format!("key-{}", Uuid::new_v4())),
                    ..Default::default()
                };
                user.insert(db).await?
            }
            Err(e) => {
                eprintln!("Error checking user {}: {}", person.name, e);
                return Err(Error::Model(e));
            }
        };
        let user_pid = user.pid;
        Ok(Some(user_pid))
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
        // the future, i should make the filtering myself, since that will allow partners to be
        // included
        let tree_nodes = create_d3_export(&family_graph, "family_data.js")
            .expect("Could not create family_data.js file");

        let mut medlem_pids = HashMap::new();
        let all_people = collect_people(&tree_nodes[0]);
        for person in all_people {
            let full_name = format!(
                "{} {}",
                person.name.clone().trim_start_matches('*').trim(),
                person.last_name.clone()
            );

            // Only creates a user with contact information and who is alive
            let user_pid = create_user(&person, &full_name, &app_context.db).await?;

            // Now we create models for all people, and ensure that the information is up to date
            let exists = medlem::Model::find_by_name(&app_context.db, &full_name).await;
            let model = match exists {
                // TODO: Should check that the fields in db correspond to current values
                Ok(model) => model,
                Err(ModelError::EntityNotFound) => {
                    // TODO
                    let (birthdate, final_date) =
                        match NaiveDate::parse_from_str(&person.birthdate, "%d.%m.%Y") {
                            Ok(date) => (Some(date), None),
                            Err(_) => {
                                let parts: Vec<&str> =
                                    person.birthdate.split('-').map(|s| s.trim()).collect();
                                if parts.len() == 2 {
                                    let parse_year = |y_str: &str| -> Option<NaiveDate> {
                                        y_str
                                            .parse::<i32>()
                                            .ok()
                                            .and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1))
                                    };
                                    let start = parse_year(parts[0]);
                                    let end = parse_year(parts[1]);
                                    (start, end)
                                } else {
                                    (None, None)
                                }
                            }
                        };
                    // Create new medlem entry
                    let new_medlem = medlem::ActiveModel {
                        pid: Set(Uuid::new_v4()),
                        user_pid: Set(user_pid),
                        name: Set(full_name.clone()),
                        email: Set(person.email.clone()),
                        birthdate: Set(birthdate),
                        final_date: Set(final_date),
                        phone_nr: Set(Some(person.mobile_number.clone())),
                        address: Set(Some(person.address.clone())),
                        city: Set(Some(person.city.clone())),
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
            // TODO: Let the user have editor privileges to the medlem
            if let Some(user_pid) = user_pid {
                println!("[INFO] Addint user as editor to medlem");
                let new_editor = medlem_editors::ActiveModel {
                    user_pid: Set(user_pid),
                    medlem_pid: Set(model.pid),
                    role: Set("editor".to_string()),
                    ..Default::default()
                };
                match new_editor.insert(&app_context.db).await {
                    Ok(_) => println!("User was added as editor for medlem"),
                    Err(e) => eprintln!(
                        "[ERROR[ An error occurred adding user as editor to medlem: {:?}",
                        e
                    ),
                }
            } else {
                eprintln!(
                    "[WARN] No user created for {}, cannot add editor privileges",
                    full_name
                );
            }
            medlem_pids.insert(full_name, model.pid);
        }

        // now with all users, the tree should then contain the full name, children and the
        // medlem_pid to be able to fetch information about the medlem, which can be changed.

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
