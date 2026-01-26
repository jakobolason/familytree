use crate::models::medlem;
use loco_rs::prelude::*;

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

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("medlem")
        // Fetch user profile
        .add("/", put(change_fields))
}
