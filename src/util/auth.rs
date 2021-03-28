use tide::{Middleware, Next, Request, Response};
use crate::util::jwt::AuthUser;

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