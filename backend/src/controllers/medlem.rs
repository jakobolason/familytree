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
    let medlem_uuid =
        Uuid::parse_str(&params.medlem_pid).map_err(|_| ModelError::EntityNotFound)?;
    let medlem = medlem::Model::find_by_pid(&ctx.db, medlem_uuid)
        .await?
        .into_active_model();

    let result = medlem
        .change_fields(&ctx.db, &auth.claims.pid, params.changeable_fields)
        .await?;
    format::json(result)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MedlemResponseData {
    pub name: String,
    pub email: Option<String>,
    pub phone_nr: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub final_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MedlemResponse {
    pub medlem: MedlemResponseData,
    // Relationship fields
    pub parents_pid: Option<serde_json::Value>,
    pub children_pid: Option<serde_json::Value>,
    pub partner_pid: Option<Uuid>,
    pub partner: Option<MedlemResponseData>,
    pub previous_partners: Option<serde_json::Value>,
}

impl From<medlem::Model> for MedlemResponseData {
    fn from(model: medlem::Model) -> Self {
        Self {
            name: model.name,
            email: Some(model.email),
            phone_nr: model.phone_nr,
            address: model.address,
            city: model.city,
            birthdate: model.birthdate,
            final_date: model.final_date,
        }
    }
}

impl From<(medlem::Model, Option<MedlemResponseData>)> for MedlemResponse {
    fn from((medlem, partner): (medlem::Model, Option<MedlemResponseData>)) -> Self {
        Self {
            medlem: medlem.clone().into(),
            parents_pid: None,  // TODO:
            children_pid: None, // TODO:
            partner_pid: medlem.partner_pid,
            partner,
            previous_partners: None, // TODO:
        }
    }
}

async fn get_medlem(
    _auth: auth::JWT,
    State(ctx): State<AppContext>,
    Path(token): Path<String>,
) -> Result<Response> {
    let pid_uuid = Uuid::parse_str(&token).map_err(|_| ModelError::EntityNotFound)?;
    let medlem = medlem::Model::find_by_pid(&ctx.db, pid_uuid).await;
    let medlem = match medlem {
        Ok(medlem) => medlem,
        Err(e) => {
            tracing::error!("Could not find medlem {:?}, {:?}", &token, e);
            return unauthorized("Unauthorized session");
        }
    };
    // If medlem has a partner_pid, fetch that also and send that together with the response
    let partner: Option<MedlemResponseData> = match medlem.partner_pid {
        Some(partner_pid) => Some(
            medlem::Model::find_by_pid(&ctx.db, partner_pid)
                .await?
                .into(),
        ),
        None => None,
    };
    format::json(MedlemResponse::from((medlem, partner)))
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("medlem")
        // Fetch user profile
        .add("/", put(change_fields))
        .add("/{token}", get(get_medlem))
}
