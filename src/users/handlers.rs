use {tide::Response, tide::prelude::*};
use crate::state::State;
use sqlx::query_as;
use crate::users::schema::{ResUser, Register};
use validator::Validate;

pub async fn register(mut req: tide::Request<State>) -> tide::Result {
    let reg_data: Register = req.body_json().await?;
    if let Err(e) = reg_data.validate() {
        tide::log::info!("{}",e);
        return  Ok(Response::from("err"));
    }
    Ok(Response::from(json!(reg_data)))
}

pub async fn login(_req: tide::Request<State>) -> tide::Result {
    Ok(Response::from(format!("login")))
}

pub async fn index(req: tide::Request<State>) -> tide::Result {
    let conn = &req.state().db;
    let result: Vec<ResUser> = query_as!(ResUser,r#"select id,name,email,phone,created_at from users"#).fetch_all(conn).await?;
    Ok(Response::from(json! {result}))
}