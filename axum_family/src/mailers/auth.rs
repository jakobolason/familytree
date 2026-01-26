use loco_rs::{
    app::AppContext,
    mailer::{self, Mailer},
    prelude::{include_dir, Dir},
    Error, Result,
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
    ///When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &user::Model) -> Result<()> {
        Self::mail_template(
            ctx,
            &FORGOT,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": ctx.config.server.full_url()
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
        Self::mail_template(
            ctx,
            &MAGIC_LINK,
            mailer::Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "token": user.magic_link_token.clone().ok_or_else(|| Error::string(
                            "the user model not contains magic link token",
                    ))?,
                  "host": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;
        Ok(())
    }
}
