use crate::models::family_tree::FAMILY_TREE_CACHE;
use crate::models::medlem_editors;
use crate::models::user;
use crate::views::auth::SessionResponse;
use loco_rs::prelude::*;

async fn current(auth: auth::JWT, State(ctx): State<AppContext>) -> Result<Response> {
    let user = user::Model::find_by_pid(&ctx.db, &auth.claims.pid).await;
    match user {
        Ok(user) => format::json(SessionResponse::new(&user)),
        Err(_) => {
            tracing::error!("Error in validing token {:?}", &auth.claims);
            unauthorized("Unauthorized session")
        }
    }
}

async fn get_authorized_medlem_pid(
    auth: auth::JWT,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user_uuid = Uuid::parse_str(&auth.claims.pid).map_err(|_| ModelError::EntityNotFound)?;
    let medlem_pids = medlem_editors::Model::who_can_user_edit(&ctx.db, &user_uuid).await?;
    format::json(medlem_pids)
}

async fn get_tree(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // tracing::error!("GUARDED ACCESS TO FAMILY TREE DATA: {:?}", _auth.claims);
    if let Ok(cache) = FAMILY_TREE_CACHE.read()
        && let Some(ref data) = *cache
    {
        return format::json(serde_json::json!(data.clone()));
    }
    Err(loco_rs::Error::Message(
        "Family tree data not available".to_string(),
    ))
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("user")
        .add("/tree", get(get_tree))
        // Fetch user profile
        .add("/current", get(current))
        .add("/medlem_pids", get(get_authorized_medlem_pid))
}
