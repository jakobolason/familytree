use crate::{
    models::_entities::user,
    views::auth::{LoginMethods, LoginResponse, SessionResponse},
};
use loco_rs::{auth::jwt, hash, prelude::*};

async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<LoginMethods>,
) -> Result<Response> {
    let user = match params {
        LoginMethods::Password(params) => {
            // Find user by email, could be moved to models/user.rs
            let user = user::Entity::find()
                .filter(user::Column::Email.eq(&params.email))
                .one(&ctx.db)
                .await?;
            let user = match user {
                Some(u) => u,
                None => return unauthorized("unauthorized!"),
            };

            // Verify password
            if !hash::verify_password(&params.password, &user.password) {
                return unauthorized("unauthorized!");
            }
            user
        }
        LoginMethods::Magic(params) => {
            println!("Logging in via magic link");
            let Ok(user) = user::Model::find_by_magic_token(&ctx.db, &params.token).await else {
                // we don't want to expose our user email. if the email is invalid we still
                // returning success to the caller
                return unauthorized("unauthorized!");
            };

            user.into_active_model().clear_magic_link(&ctx.db).await?
        }
    };

    tracing::info!("User {} logged in", user.email);
    println!("user login method verified!");
    // Generate the JWT
    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = jwt::JWT::new(&jwt_secret.secret)
        .generate_token(
            jwt_secret.expiration,
            user.pid.to_string(),
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
    let user = user::Model::find_by_pid(&ctx.db, &auth.claims.pid).await;
    match user {
        Ok(user) => format::json(SessionResponse::new(&user)),
        Err(_) => {
            tracing::error!("Error in validing token {:?}", &auth.claims);
            unauthorized("Unauthorized session")
        }
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
