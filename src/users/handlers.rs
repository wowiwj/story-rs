use {tide::Response};
use crate::state::State;
use sqlx::query_as;
use crate::users::schema::{ResUser, Register};
use validator::Validate;
use crate::util::api::Api;
use crate::util::status;

pub async fn register(mut req: tide::Request<State>) -> tide::Result {
    let reg_data: Register = req.body_json().await?;
    if let Err(e) = reg_data.validate() {
        tide::log::info!("{}", e);
        return Api::builder(&status::BAD_REQUEST).errors(e).response();
    }
    Api::success(Some(reg_data))
    // Ok(Response::from(json!(reg_data)))
}

pub async fn login(_req: tide::Request<State>) -> tide::Result {
    Ok(Response::from(format!("login")))
}

pub async fn index(req: tide::Request<State>) -> tide::Result {
    let conn = &req.state().db;
    let result: Vec<ResUser> = query_as!(ResUser,r#"select id,name,email,phone,created_at from users"#).fetch_all(conn).await?;
    Api::success(Some(result))
}