use crate::models::family_tree::FAMILY_TREE_CACHE;
use loco_rs::prelude::*;

async fn current(auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // Give the JWT is valid, return the user profile
    format::json(serde_json::json!({
        "pid": auth.claims.pid,
        "name": auth.claims.pid,
        "email": auth.claims.pid,
    }))
}

async fn get_tree(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    // tracing::error!("UNGUARDED ACCESS TO FAMILY TREE DATA: {:?}", auth.claims);
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
