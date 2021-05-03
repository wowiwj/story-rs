use tide::{Middleware, Next, Request, Response};

use crate::state::State;
use std::option::Option::Some;

use crate::api::status;
use crate::api::error::ApiErr;
use crate::jwt::jwt::{AuthUser, Claims};
use tide::http::headers::HeaderValues;
use jsonwebtoken::{DecodingKey, TokenData, decode, Validation, encode, Header, EncodingKey};


pub struct Auth {
    secret: String,
}

impl Auth {
    pub fn new(secret: String) -> Self {
        Self {
            secret
        }
    }

    pub fn check(req: &tide::Request<State>) -> Result<&AuthUser, ApiErr> {
        if let Some(user) = req.ext::<AuthUser>() {
            return Ok(user);
        }
        Err(ApiErr::status(&status::UNAUTH))
    }

    pub fn get_secret(&self) -> String {
        String::from(&self.secret)
    }

    pub fn create_token(&self, user: &AuthUser) -> jsonwebtoken::errors::Result<String> {
        let claims = Claims::from(user);
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.get_secret().as_bytes()))
    }


    pub fn parse_token(&self, header: Option<&HeaderValues>) -> Option<AuthUser> {
        let token = match Self::exact_token(header) {
            Some(token) => token,
            None => return None
        };

        let decode = decode(&token, &DecodingKey::from_secret(&self.get_secret().as_bytes()), &Validation::default());

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


pub struct JwtMiddleWare {
    state: State,
}

impl JwtMiddleWare {
    pub fn new(state: State) -> Self {
        Self {
            state
        }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for JwtMiddleWare {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> tide::Result<Response> {
        tide::log::info!("Headers: {:?}", request.header("Authorization"));

        let auth = Auth::new(self.state.jwt_secret());
        if let Some(u) = auth.parse_token(request.header("Authorization")) {
            tide::log::info!("User: {:?}", u);
            request.set_ext(u);
        }
        let res = next.run(request).await;
        Ok(res)
    }
}