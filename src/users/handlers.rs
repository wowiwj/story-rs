use {tide::Response};
use crate::state::State;
use sqlx::query_as;
use crate::users::schema::{ResUser, Register, ResAuthUser};
use validator::Validate;
use crate::util::api::{Api, ApiErr};
use crate::models::users::User;
use crate::util::jwt::AuthUser;


pub async fn register(mut req: tide::Request<State>) -> tide::Result {
    let reg_data: Register = req.body_json().await?;
    if let Err(e) = reg_data.validate() {
        return Api::error_validate(e);
    }
    let conn = &req.state().db;

    if let Ok(_row) = sqlx::query(r#"select * from users where email = ? and deleted_at is null"#)
        .bind(&reg_data.email)
        .fetch_one(conn).await {
        return Api::error(ApiErr::builder()
            .add("email", "用户邮箱已存在")
            .build()
        );
    }
    let mut  user = User::from(reg_data);
    let id = user.create(conn).await?;
    user.id = id;
    let token = AuthUser::from(&user).create_token()?;

    Api::success(ResAuthUser{
        user: ResUser::from(user),
        token: token
    })
}

pub async fn login(_req: tide::Request<State>) -> tide::Result {
    Ok(Response::from(format!("login")))
}

pub async fn index(req: tide::Request<State>) -> tide::Result {
    let conn = &req.state().db;
    let result: Vec<ResUser> = query_as!(ResUser,r#"select id,name,email,phone,created_at from users where deleted_at is null"#)
        .fetch_all(conn)
        .await?;
    Api::success(result)
}