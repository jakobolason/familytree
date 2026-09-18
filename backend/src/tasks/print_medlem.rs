use loco_rs::prelude::*;

use crate::models::_entities::medlem;

pub struct PrintMedlem;
#[async_trait]
impl Task for PrintMedlem {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "print_medlem".to_string(),
            detail: "Print the current version of a medlem's data".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        let name = vars.cli_arg("name").expect("You have to supply a name");

        let model = medlem::Model::find_by_name(&app_context.db, name).await?;
        print!("{:?}", model);
        Ok(())
    }
}
