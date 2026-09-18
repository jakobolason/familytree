use crate::models::medlem::ChangeableFields;
use loco_rs::prelude::*;

use crate::models::{_entities::medlem, medlem::check_discrepancy};

#[allow(clippy::module_name_repetitions)]
pub struct AlterMedlem;
#[async_trait]
impl Task for AlterMedlem {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "alter_medlem".to_string(),
            detail: "Task for editing a specific member\n\
                    \t\t\tUsage: cargo loco task alter_medlem full_name:\"John Doe\" \n\
                    \t\t\t\t[email:new@email.com] [number:123] [new_name:\"New Name\"] \n\
                    \t\t\t\t[address:\"New Address\"] [city:\"New City\"] [birthdate:1999-01-01]"
                .to_string(),
        }
    }

    async fn run(&self, app_context: &AppContext, vars: &task::Vars) -> Result<()> {
        // Try and find model from name or PID, checks PID first
        let model = if let Ok(pid) = vars.cli_arg("pid") {
            let user_uuid = Uuid::parse_str(pid).map_err(|_| ModelError::EntityNotFound)?;
            medlem::Model::find_by_pid(&app_context.db, user_uuid).await?
        } else if let Ok(name) = vars.cli_arg("name") {
            medlem::Model::find_by_name(&app_context.db, name).await?
        } else {
            return Err(Error::Message("No name or PID given".to_string()));
        };
        let person = Person {
            name: vars
                .cli_arg("name")
                .expect("A name should always be provided"),
            birthdate: vars.cli_arg("birthdate").ok(),
            final_date: vars.cli_arg("final_date").ok(),
            address: vars.cli_arg("address").ok(),
            city: vars.cli_arg("city").ok(),
            mobile_number: vars.cli_arg("number").ok(),
            email: vars.cli_arg("email").ok(),
        };
        let possible_discrepancies = found_discrepancies(person, &model, true);
        let discrepancies = if let Some(discrepancies) = possible_discrepancies {
            discrepancies
        } else {
            eprintln!("\n\tNo changes detected!");
            return Ok(());
        };
        model
            .into_active_model()
            .change_fields(&app_context.db, discrepancies)
            .await?;
        Ok(())
    }
}

pub struct Person<'a> {
    pub name: &'a str,
    pub birthdate: Option<&'a str>,
    pub final_date: Option<&'a str>,
    pub address: Option<&'a str>,
    pub city: Option<&'a str>,
    pub mobile_number: Option<&'a str>,
    pub email: Option<&'a str>,
}

fn found_discrepancies(
    person: Person,
    model: &medlem::Model,
    overwrite: bool,
) -> Option<ChangeableFields> {
    let name = check_discrepancy(person.name, Some(&model.name), "Name", overwrite);
    let phone_nr = if let Some(number) = person.mobile_number {
        check_discrepancy(
            number,
            model.phone_nr.as_deref(),
            "Mobile number",
            overwrite,
        )
    } else {
        None
    };
    let address = if let Some(address) = person.address {
        check_discrepancy(address, model.address.as_deref(), "Address", overwrite)
    } else {
        None
    };
    let city = if let Some(city) = person.city {
        check_discrepancy(city, model.city.as_deref(), "City", overwrite)
    } else {
        None
    };
    let email = if let Some(email) = person.email {
        check_discrepancy(email, Some(&model.email), "Email", overwrite)
    } else {
        None
    };
    let date_formatting = |d: Option<Date>| d.map(|b| b.format("%Y-%m-%d").to_string());
    let birthdate = if let Some(birthdate) = person.birthdate {
        check_discrepancy(
            birthdate,
            date_formatting(model.birthdate).as_deref(),
            "Birthdate",
            overwrite,
        )
    } else {
        None
    };

    let final_date = if let Some(final_date) = person.final_date {
        check_discrepancy(
            final_date,
            date_formatting(model.final_date).as_deref(),
            "Final date",
            overwrite,
        )
    } else {
        None
    };

    if name.is_none()
        && phone_nr.is_none()
        && address.is_none()
        && birthdate.is_none()
        && city.is_none()
        && email.is_none()
    {
        None
    } else {
        Some(ChangeableFields {
            phone_nr: phone_nr.map(|p| p.to_string()),
            address: address.map(|a| a.to_string()),
            birthdate: birthdate.map(|b| b.to_string()),
            final_date: final_date.map(|f| f.to_string()),
            email: email.map(|e| e.to_string()),
            city: city.map(|c| c.to_string()),
            name: name.map(|n| n.to_string()),
        })
    }
}
