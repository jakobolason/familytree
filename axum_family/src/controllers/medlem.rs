use crate::models::{medlem, medlem_editors};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Changefields {
    medlem_pid: String,
    changeable_fields: medlem::ChangeableFields,
}

#[axum::debug_handler]
async fn change_fields(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<Changefields>,
) -> Result<Response> {
    // Check that user has edit priveleges
    let user_uuid = Uuid::parse_str(&auth.claims.pid).map_err(|_| ModelError::EntityNotFound)?;
    let medlem_uuid =
        Uuid::parse_str(&params.medlem_pid).map_err(|_| ModelError::EntityNotFound)?;
    if medlem_editors::Model::is_user_editor(&ctx.db, &medlem_uuid, &user_uuid)
        .await
        .is_err()
    {
        tracing::error!(
            "User {:?} is not authorized to edit medlem {:?}",
            &auth.claims,
            &params.medlem_pid
        );
        return unauthorized("You cannot edit this user");
    }
    // let user = user::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let medlem = medlem::Model::find_by_pid(&ctx.db, &params.medlem_pid)
        .await?
        .into_active_model();

    let result = medlem
        .change_fields(&ctx.db, &auth.claims.pid, params.changeable_fields)
        .await?;
    format::json(result)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MedlemResponse {
    pub name: String,
    pub email: Option<String>,
    pub phone_nr: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub final_date: Option<NaiveDate>,
    pub status: String,
    // Relationship fields
    pub parents_pid: Option<serde_json::Value>,
    pub children_pid: Option<serde_json::Value>,
    pub partner_pid: Option<Uuid>,
    pub previous_partners: Option<serde_json::Value>,
}

impl From<medlem::Model> for MedlemResponse {
    fn from(model: medlem::Model) -> Self {
        Self {
            name: model.name,
            email: Some(model.email),
            phone_nr: model.phone_nr,
            address: model.address,
            city: model.city,
            birthdate: model.birthdate,
            final_date: model.final_date,
            status: model.status,
            parents_pid: model.parents_pid,
            children_pid: model.children_pid,
            partner_pid: model.partner_pid,
            previous_partners: model.previous_partners,
        }
    }
}

async fn get_medlem(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(token): Path<String>,
) -> Result<Response> {
    tracing::info!("Fetching medlem profile for pid: {}", &token);
    let medlem = medlem::Model::find_by_pid(&ctx.db, &token).await;
    match medlem {
        Ok(medlem) => format::json(MedlemResponse::from(medlem)),
        Err(_) => {
            tracing::error!("Could not find medlem {:?}", &auth.claims);
            unauthorized("Unauthorized session")
        }
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("medlem")
        // Fetch user profile
        .add("/", put(change_fields))
        .add("/{token}", get(get_medlem))
}
