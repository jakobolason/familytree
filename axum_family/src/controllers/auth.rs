use crate::models::_entities::user;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordLoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    pub fn new(user: &user::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub pid: String,
    pub name: String,
}
impl SessionResponse {
    pub fn new(user: &user::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
        }
    }
}

async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<PasswordLoginParams>,
) -> Result<Response> {
    // Find user by email, could be moved to models/user.rs
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&params.email))
        .one(&ctx.db)
        .await?;
    let Some(user) = user else {
        return unauthorized("unauthorized!");
    };

    // Verify password
    if !hash::verify_password(&params.password, &user.password) {
        return unauthorized("unauthorized!");
    }

    // Generate the JWT
    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = jwt::JWT::new(&jwt_secret.secret)
        .generate_token(
            jwt_secret.expiration,
            params.email.to_string(),
            Default::default(),
        )
        .map_err(|e| {
            tracing::error!("JWT generation error: {:?}", e);
            loco_rs::Error::InternalServerError
        })?;

    // Login success
    format::json(LoginResponse::new(&user, &token))
}

async fn get_session(State(ctx): State<AppContext>, auth: auth::JWT) -> Result<Response> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&auth.claims.pid))
        .one(&ctx.db)
        .await?;
    if let Some(user) = user {
        format::json(SessionResponse::new(&user))
    } else {
        tracing::error!("Error in validing token {:?}", &auth.claims);
        unauthorized("Unauthorized session")
    }
}

pub fn routes() -> Routes {
    Routes::new()
        // Authentication route prefix
        .prefix("auth")
        // Handling login with password
        .add("/login", post(login))
        .add("/session", get(get_session))
}
