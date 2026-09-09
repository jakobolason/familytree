use crate::models::{
    _entities::user, family_tree::ActiveModel as FamilyTreeActive, medlem, medlem_editors,
};
use chrono::{NaiveDate, Utc};
use family_graph::{
    Relationship,
    family_graph::{FamilyGraph, Person},
    run_grapher,
};
use loco_rs::{hash, prelude::*};
use petgraph::{Direction, graph::NodeIndex, visit::EdgeRef};
use std::{collections::HashMap, path::Path};

pub const RANDOM_PASSWD_LENGTH: i8 = 20;
const PIVOT: u32 = 26;

// Only describes a person and their immediate children
#[derive(serde::Serialize, serde::Deserialize)]
pub struct FamilyTreeNode {
    name: String,
    pid: Option<Uuid>,
    children: Vec<FamilyTreeNode>,
    partner: Option<String>,
}

// To create the tree (full tree from root)
#[derive(Debug)]
struct D3Node {
    person: Person,
    partner: Option<Person>,
    children: Vec<D3Node>,
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
            partner: None,
        }
    }
}

// Should return (Person, Option<Partner>)
fn collect_people(node: &D3Node) -> Vec<(Person, Option<Person>)> {
    let mut people = vec![(node.person.clone(), node.partner.clone())];

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
                tracing::info!("User {full_name} exists");
                user
            }
            Err(ModelError::EntityNotFound) => {
                // NOTE: All users password's are initialized as random
                let random_str = hash::random_string(RANDOM_PASSWD_LENGTH as usize);
                let distinct_str = format!(
                    "{}-{}-{}",
                    random_str,
                    full_name,
                    Utc::now().timestamp_millis()
                );
                let hashed_password = hash::hash_password(&distinct_str)
                    .map_err(|e| Error::Message(format!("Password hashing error: {}", e)))?;

                let user = user::ActiveModel {
                    pid: Set(Uuid::new_v4()),
                    name: Set(full_name.to_string()),
                    email: Set(person.email.clone()),
                    password: Set(hashed_password),
                    api_key: Set(format!("key-{}", Uuid::new_v4())),
                    ..Default::default()
                };
                user.insert(db).await?
            }
            Err(e) => {
                tracing::error!("Error checking user {}: {}", person.name, e);
                return Err(Error::Model(e));
            }
        };
        let user_pid = user.pid;
        Ok(Some(user_pid))
    }
}

// This function handles creating the recursive tree for a root node, and getting each
// node's partner along the way. This means each root contains the children of all
// descendents, while descendants only contain the children beneath them
fn recursive_children(family: &FamilyGraph, node: NodeIndex) -> D3Node {
    tracing::info!(
        "Adding node for {} {}",
        family[node].name,
        family[node].last_name,
    );
    let children: Vec<D3Node> = family
        .edges_directed(node, Direction::Outgoing)
        .filter(|edge| matches!(edge.weight(), Relationship::Child))
        .map(|edge| {
            let child_idx = edge.target();
            recursive_children(family, child_idx)
        })
        .collect();
    let partner = family
        .edges_directed(node, Direction::Outgoing)
        .find(|edge| matches!(edge.weight(), Relationship::Married))
        .map(|e| family[e.target()].clone());
    tracing::info!("Found partner: {:?}", partner);
    D3Node {
        person: family[node].clone(),
        children,
        partner,
    }
}

fn parse_birthdate(birthdate: &str) -> Option<NaiveDate> {
    let parts: Vec<&str> = birthdate.split(".").collect();
    if parts.len() != 3 {
        return None;
    }
    let day: u32 = parts[0].parse().ok()?;
    let month = parts[1].parse().ok()?;
    let year_str = parts[2];
    let year: u32 = year_str.parse().ok()?;
    let full_year = if year_str.len() == 4 {
        year
    } else {
        if year <= PIVOT {
            year + 2000
        } else {
            year + 1900
        }
    };

    NaiveDate::from_ymd_opt(full_year as i32, month, day)
}

fn check_discrepancy<'a>(
    given: &'a str,
    found: &Option<String>,
    name: &str,
    overwrite: bool,
) -> Option<&'a str> {
    if let Some(found) = found
        && given != found
    {
        if overwrite {
            Some(given)
        } else {
            tracing::info!(
                "{} discrepancy found! given: {}, found: {}",
                name,
                given,
                found
            );
            None
        }
    } else {
        None
    }
}

fn found_discrepancies(
    person: Person,
    model: &medlem::Model,
    overwrite: bool,
) -> Option<medlem::ChangeableFields> {
    let name = check_discrepancy(&person.name, &Some(model.name.clone()), "Name", overwrite);
    let phone_nr = check_discrepancy(
        &person.mobile_number,
        &model.phone_nr,
        "Mobile number",
        overwrite,
    );
    let address = check_discrepancy(&person.address, &model.address, "Address", overwrite);
    let birthdate = check_discrepancy(
        &person.birthdate,
        &model.birthdate.map(|b| b.format("%Y-%m-%d").to_string()),
        "Birthdate",
        overwrite,
    );
    let city = check_discrepancy(&person.city, &model.city, "City", overwrite);
    let email = check_discrepancy(
        &person.email,
        &Some(model.email.clone()),
        "Email",
        overwrite,
    );

    if name.is_none()
        && phone_nr.is_none()
        && address.is_none()
        && birthdate.is_none()
        && city.is_none()
        && email.is_none()
    {
        None
    } else {
        Some(medlem::ChangeableFields {
            phone_nr: phone_nr.map(|p| p.to_string()),
            address: address.map(|a| a.to_string()),
            birthdate: birthdate.map(|b| b.to_string()),
            email: email.map(|e| e.to_string()),
            city: city.map(|c| c.to_string()),
            name: name.map(|n| n.to_string()),
        })
    }
}

async fn create_medlem(
    db: &DatabaseConnection,
    full_name: String,
    person: Person,
    user_pid: Option<Uuid>,
    partner_pid: Option<Uuid>,
    overwrite: bool,
) -> Result<medlem::Model> {
    // Now we create models for all people, and ensure that the information is up to date
    let exists = medlem::Model::find_by_name(db, &full_name).await;
    let model = match exists {
        Ok(model) => {
            if let Some(changeable_fields) = found_discrepancies(person, &model, overwrite) {
                let model = model
                    .into_active_model()
                    .change_fields(db, "123", changeable_fields)
                    .await?;
                return Ok(model);
            }
            model
        }
        Err(ModelError::EntityNotFound) => {
            let (birthdate, final_date) = match parse_birthdate(&person.birthdate) {
                Some(date) => (Some(date), None),
                None => {
                    let parts: Vec<&str> = person.birthdate.split('-').map(|s| s.trim()).collect();
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
                partner_pid: Set(partner_pid),
                ..Default::default()
            };
            match new_medlem.insert(db).await {
                Ok(model) => {
                    tracing::info!("Created medlem entry for {}", full_name);
                    model
                }
                Err(e) => {
                    tracing::error!("Error creating medlem for {}: {}", full_name, e);
                    return Err(e.into());
                }
            }
        }
        Err(e) => {
            tracing::error!("Error finding medlem for {}: {}", full_name, e);
            return Err(e.into());
        }
    };
    if let Some(user_pid) = user_pid {
        tracing::info!("[INFO] Adding user as editor to medlem");
        let new_editor = medlem_editors::ActiveModel {
            user_pid: Set(user_pid),
            medlem_pid: Set(model.pid),
            role: Set("editor".to_string()),
            ..Default::default()
        };
        match new_editor.insert(db).await {
            Ok(_) => tracing::info!("User was added as editor for medlem"),
            Err(e) => {
                tracing::error!(
                    "[ERROR[ An error occurred adding user as editor to medlem: {:?}",
                    e
                );
                return Err(e.into());
            }
        }
    } else {
        tracing::error!(
            "[WARN] No user created for {}, cannot add editor privileges",
            full_name
        );
    }
    Ok(model)
}

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
        tracing::info!("Family Tree D3 Export Task...");
        let path = vars.cli_arg("path");
        let overwrite = vars.cli_arg("overwrite").is_ok();
        tracing::info!("Overwrite is set to: {}", overwrite);

        let default_path = "family_data.xls";
        let file_path = match path {
            Ok(path) => path,
            Err(_) => {
                tracing::info!("No file specified, defaulting to '{}'", default_path);
                default_path
            }
        };

        let path = Path::new(file_path);

        if !path.exists() {
            tracing::error!("Error: File '{}' not found.", file_path);
            std::process::exit(1);
        }

        let family_graph = match run_grapher(path, "Ark1") {
            Ok(graph) => graph,
            Err(e) => {
                tracing::error!("Error generating tree: {}", e);
                std::process::exit(1);
            }
        };

        // FamilyGraph => D3 tree
        let tree_data: Vec<D3Node> = family_graph
            .node_indices()
            .filter(|&node_idx| {
                !family_graph
                    .edges_directed(node_idx, Direction::Incoming)
                    .any(|edge| matches!(edge.weight(), Relationship::Child))
                    // also add partners
                    && !family_graph
                        .edges_directed(node_idx, Direction::Incoming)
                        .any(|edge| matches!(edge.weight(), Relationship::Married))
            })
            .map(|node| recursive_children(&family_graph, node))
            .collect();
        tracing::info!("\n\nGenerated D3 Tree: {:?}", tree_data);
        // full_name => medlem pid
        let mut medlem_pids = HashMap::new();
        let all_people = collect_people(&tree_data[0]);
        for (person, partner) in all_people {
            // Should first create partner user and model, and then link the partner_pid to the
            // person's partner field
            let partner_pid: Option<Uuid> = if let Some(partner) = partner {
                let full_name = format!(
                    "{} {}",
                    partner.name.clone().trim_start_matches('~').trim(),
                    partner.last_name.clone()
                );

                // Only creates a user with contact information and who is alive
                let user_pid = create_user(&partner, &full_name, &app_context.db).await?;

                // Now we create models for all people, and ensure that the information is up to date
                let model = match create_medlem(
                    &app_context.db,
                    full_name.clone(),
                    partner.clone(),
                    user_pid,
                    None,
                    overwrite,
                )
                .await
                {
                    Ok(model) => model,
                    Err(e) => {
                        tracing::error!("Error creating medlem for {}: {}", full_name, e);
                        continue;
                    }
                };
                Some(model.pid)
            } else {
                None
            };

            let full_name = format!(
                "{} {}",
                person.name.clone().trim_start_matches('*').trim(),
                person.last_name.clone()
            );

            // Only creates a user with contact information and who is alive
            let user_pid = create_user(&person, &full_name, &app_context.db).await?;

            // Now we create models for all people, and ensure that the information is up to date
            let model = match create_medlem(
                &app_context.db,
                full_name.clone(),
                person.clone(),
                user_pid,
                partner_pid,
                overwrite,
            )
            .await
            {
                Ok(model) => model,
                Err(e) => {
                    tracing::error!("Error creating medlem for {}: {}", full_name, e);
                    continue;
                }
            };

            medlem_pids.insert(full_name, model.pid);
        }

        // now with all users, the tree should then contain the full name, children and the
        // medlem_pid to be able to fetch information about the medlem, which can be changed.

        // Strips tree_nodes of information, such that only tree remains
        let only_tree = tree_data
            .first()
            .map(|n| FamilyTreeNode::create_d3_tree(n, &medlem_pids))
            .expect("The first entry should be the forefather, who is not present");

        let json_value = serde_json::to_value(&only_tree)?;
        // Now save the tree in memory
        FamilyTreeActive::create_snapshot(&app_context.db, json_value)
            .await
            .map_err(|e| Error::Message(format!("JSON serialization error: {}", e)))?;

        tracing::info!("Tree generated successfully!");

        Ok(())
    }
}
