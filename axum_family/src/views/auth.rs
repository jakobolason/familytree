use serde::{Deserialize, Serialize};

use crate::models::_entities::user;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &user::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub name: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &user::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordLoginParams {
    pub email: String,
    pub password: String,
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
