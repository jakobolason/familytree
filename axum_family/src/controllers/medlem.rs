use crate::models::medlem;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[axum::debug_handler]
async fn change_fields(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<medlem::ChangeableFields>,
) -> Result<Response> {
    // let user = user::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;
    let medlem = medlem::Model::find_by_pid(&ctx.db, &auth.claims.pid)
        .await?
        .into_active_model();

    let result = medlem
        .change_fields(&ctx.db, &auth.claims.pid, params)
        .await?;
    format::json(result)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MedlemParams {
    pub medlem_pid: String,
}
async fn get_user(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
    Json(params): Json<MedlemParams>,
) -> Result<Response> {
    let medlem = medlem::Model::find_by_pid(&ctx.db, &params.medlem_pid).await;
    match medlem {
        Ok(medlem) => format::json(medlem),
        Err(_) => {
            tracing::error!("Error in validing token {:?}", &auth.claims);
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
        .add("/", get(get_user))
}
