use chrono::{Local, Duration};
use jsonwebtoken::{encode, Header, EncodingKey, decode, DecodingKey, Validation, TokenData};

use tide::prelude::*;
use tide::http::headers::HeaderValues;


#[derive(Deserialize, Debug)]
pub struct AuthUser {
    pub id: u64,
    pub username: String,
    pub email: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    username: String,
    uid: u64,
    exp: usize,
}


impl Into<AuthUser> for Claims {
    fn into(self) -> AuthUser {
        AuthUser {
            id: self.uid,
            username: self.username,
            email: self.sub,
        }
    }
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
        String::from("test")
    }

    pub fn create_token(&self) -> jsonwebtoken::errors::Result<String> {
        let claims = Claims::from(self);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(AuthUser::get_secret().as_bytes()))
    }

    pub fn parse_token(header: Option<&HeaderValues>) -> Option<AuthUser> {
        let token = match Self::exact_token(header) {
            Some(token) => token,
            None => return None
        };

        let decode = decode(&token, &DecodingKey::from_secret(Self::get_secret().as_bytes()), &Validation::default());

        if let Err(ref e) = decode {
            tide::log::info!("{}", e);
        }

        decode.map(|token_data: TokenData<Claims>| {
            token_data.claims.into()
        }).ok()
    }

    pub fn exact_token(header: Option<&HeaderValues>) -> Option<String> {
        header.and_then(|h| {
            h.as_str().split(' ').nth(1)
        })
            .and_then(|token| {
                Some(String::from(token))
            }).or(None)
    }
}