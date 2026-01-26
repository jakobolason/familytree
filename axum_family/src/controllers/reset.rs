use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::user,
        // user::{LoginParams, RegisterParams},
    },
    // views::auth::{CurrentResponse, LoginResponse},
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing user DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(user) = user::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our user email. if the email is invalid we still
        // returning success to the caller
        return format::json(());
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    format::json(())
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = user::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our user email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        // User route prefix
        .prefix("reset")
        .add("/", get(reset))
        .add("/forgot", get(forgot))
    // Fetch user profile
}
