use loco_rs::prelude::*;
use std::path::Path;

use crate::{
    grapher::graph_creater::run_grapher,
    models::family_tree::{ActiveModel as FamilyTreeActive, FamilyTree},
};

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

        println!("Tree generated successfully!");

        Ok(())
    }
}
