use crate::{
    mailers::auth::AuthMailer,
    models::{
        _entities::user,
        // user::{LoginParams, RegisterParams},
    },
    views::auth::LoginResponse,
};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use validator as val;

#[derive(Debug, Deserialize, val::Validate, Serialize)]
pub struct MagicLinkParams {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Deserialize, val::Validate, Serialize)]
pub struct MagicLinkToken {
    #[validate(email)]
    pub token: String,
}

async fn magic_link(
    State(ctx): State<AppContext>,
    Json(params): Json<MagicLinkParams>,
) -> Result<Response> {
    if let Err(e) = val::Validate::validate(&params) {
        tracing::debug!(
            email = params.email,
            "The provided email is invalid or does not match the allowed domains: {:?}",
            e
        );
        return bad_request("invalid request");
    }

    let Ok(user) = user::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our user email. if the email is invalid we still
        // returning success to the caller
        tracing::debug!(email = params.email, "user not found by email");
        return format::empty_json();
    };
    tracing::debug!("user found by email: {}", user.email);

    let user = user.into_active_model().create_magic_link(&ctx.db).await?;
    AuthMailer::send_magic_link(&ctx, &user).await?;

    format::empty_json()
}

/// Verifies a magic link token and authenticates the user.
async fn magic_link_verify(
    State(ctx): State<AppContext>,
    Json(params): Json<MagicLinkToken>,
) -> Result<Response> {
    let Ok(user) = user::Model::find_by_magic_token(&ctx.db, &params.token).await else {
        // we don't want to expose our user email. if the email is invalid we still
        // returning success to the caller
        return unauthorized("unauthorized!");
    };

    let user = user.into_active_model().clear_magic_link(&ctx.db).await?;

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    format::json(LoginResponse::new(&user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("magic-link")
        .add("/", post(magic_link))
        .add("/verify", get(magic_link_verify))
}
