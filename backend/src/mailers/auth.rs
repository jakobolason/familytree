use loco_rs::{
    Error, Result,
    app::AppContext,
    mailer::{self, Mailer},
    prelude::{Dir, include_dir},
};
use serde_json::json;

use crate::models::user;

//static welcome: Dir<'_> = include_dir!("src/mailers/auth/welcome");
static FORGOT: Dir<'_> = include_dir!("src/mailers/auth/forgot");
static MAGIC_LINK: Dir<'_> = include_dir!("src/mailers/auth/magic_link");

pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending forgot password email
    ///
    /// # Errors
    ///
    ///When email sending fails
    pub async fn forgot_password(ctx: &AppContext, user: &user::Model) -> Result<()> {
        let frontend_url = ctx
            .config
            .settings
            .as_ref()
            .and_then(|s| s.get("frontend_url"))
            .and_then(|v| v.as_str())
            .unwrap_or("http://localhost:5173");
        Self::mail_template(
            ctx,
            &FORGOT,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": frontend_url
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sends a magic link authentication mail to the user
    ///
    /// # Errors
    ///
    /// When email sending, which can fail
    pub async fn send_magic_link(ctx: &AppContext, user: &user::Model) -> Result<()> {
        let frontend_url = ctx
            .config
            .settings
            .as_ref()
            .and_then(|s| s.get("frontend_url"))
            .and_then(|v| v.as_str())
            .unwrap_or("http://localhost:3000");
        let from_email = ctx
            .config
            .settings
            .as_ref()
            .and_then(|s| s.get("from_email"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        tracing::debug!(
            "Sending magic link email to {} from {:?}",
            user.email,
            from_email
        );
        Self::mail_template(
            ctx,
            &MAGIC_LINK,
            mailer::Args {
                from: from_email,
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model does not contains magic link token",
                    ))?,
                  "host": frontend_url
                }),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }
}
