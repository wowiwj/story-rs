use tide::{Middleware, Next, Request, Response};
use crate::util::jwt::AuthUser;

use crate::state::State;
use std::option::Option::Some;

use crate::util::status;
use crate::util::error::ApiErr;


pub struct Auth{}

impl Auth {
    pub fn check(req: &tide::Request<State>) -> Result<&AuthUser, ApiErr> {
        if let Some(user) = req.ext::<AuthUser>() {
            return Ok(user);
        }
        Err(ApiErr::status(&status::UNAUTH))
    }
}




pub struct JwtMiddleWare {}

impl JwtMiddleWare {
    pub fn new() -> Self {
        Self {}
    }
}
#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for JwtMiddleWare {
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> tide::Result<Response> {
        tide::log::info!("Headers: {:?}", request.header("Authorization"));
        if let Some(u) = AuthUser::parse_token(request.header("Authorization")) {
            tide::log::info!("User: {:?}", u);
            request.set_ext(u);
        }
        let res = next.run(request).await;
        Ok(res)
    }
}