use crate::models::family_tree::FAMILY_TREE_CACHE;
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

async fn get_tree(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // tracing::error!("GUARDED ACCESS TO FAMILY TREE DATA: {:?}", _auth.claims);
    if let Ok(cache) = FAMILY_TREE_CACHE.read() {
        if let Some(ref data) = *cache {
            return format::json(serde_json::json!(data.clone()));
        }
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
}
