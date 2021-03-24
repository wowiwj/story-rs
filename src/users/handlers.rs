use {tide::Response,tide::prelude::*};
use crate::state::State;
use sqlx::query_as;
use crate::users::schema::ResUser;

pub async fn index(req: tide::Request<State>) -> tide::Result {
    let conn = &req.state().db;
    let result: Vec<ResUser> = query_as!(ResUser,r#"select id,name,email,phone,created_at from users"#).fetch_all(conn).await?;
    Ok(Response::from(json! {result}))
}