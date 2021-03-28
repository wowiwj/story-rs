use chrono::{Local, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::CONFIG;

use crate::models::users::User;
use tide::prelude::*;

pub struct AuthUser {
    id: u64,
    username: String,
    email: String,
}

impl From<&User> for AuthUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.name.to_string(),
            email: user.email.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    username: String,
    uid: u64,
    exp: usize,
}

impl From<&AuthUser> for Claims {
    fn from(u: &AuthUser) -> Self {
        Self {
            sub: u.email.to_string(),
            username: u.username.to_string(),
            uid: u.id,
            exp: (Local::now() + Duration::hours(24)).timestamp() as usize,
        }
    }
}

impl AuthUser {
    pub fn get_secret() -> String {
        String::from(&CONFIG.server.jwt_secret)
    }

    pub fn create_token(&self) -> jsonwebtoken::errors::Result<String> {
        let claims = Claims::from(self);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(AuthUser::get_secret().as_bytes()))
    }
}