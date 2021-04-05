
use crate::state::State;
use sqlx::{query_as, MySqlPool};
use crate::users::schema::{ResUser, Register, ResAuthUser, Login};
use validator::Validate;
use crate::util::api::{Api};
use crate::models::users::User;
use crate::util::jwt::{AuthUser};
use std::result::Result::Err;
use crate::util::crypt::password_verify;
use crate::util::error::ApiErr;
use crate::util::auth::Auth;
use quaint::ast::Select;
use quaint::prelude::*;
use quaint::visitor::{Mysql, Visitor};
use crate::query::StQuery;


pub async fn register(mut req: tide::Request<State>) -> tide::Result {
    let reg_data: Register = req.body_json().await?;
    if let Err(e) = reg_data.validate() {
        return Api::error_validate(e);
    }
    let conn = &req.state().db;

    if  User::find_by_email(&reg_data.email, conn).await.is_ok() {
        return ApiErr::builder()
            .add("email", "用户邮箱已存在")
            .build();
    }
    let mut user = User::from(reg_data);
    let id = user.create(conn).await?;
    user.id = id;
    let token = AuthUser::from(&user).create_token()?;

    Api::success(ResAuthUser {
        user: ResUser::from(user),
        token: token,
    })
}

pub async fn login(mut req: tide::Request<State>) -> tide::Result {
    let login_data: Login = req.body_json().await?;
    if let Err(e) = login_data.validate() {
        return Api::error_validate(e);
    }
    let conn = &req.state().db;
    let user = match User::find_by_email(&login_data.email, conn).await {
        Ok(u) => u,
        Err(_) => return ApiErr::builder().add("email", "当前用户不存在").build()
    };
    if !password_verify(&user.password, &login_data.password) {
        return ApiErr::builder()
            .add("password", "用户名或密码不存在")
            .build();
    }
    let token = AuthUser::from(&user).create_token()?;
    Api::success(ResAuthUser {
        user: ResUser::from(user),
        token: token,
    })
}

pub async fn index(req: tide::Request<State>) -> tide::Result {
    Auth::check(&req)?;
    let conn = &req.state().db;
    let select = Select::from_table("users")
        .columns(&["id", "name","phone", "email", "created_at"])
        .so_that("deleted_at".is_null());
    let (sql_str, _) = Mysql::build(select)?;
    let result: Vec<ResUser> = sqlx::query_as::<_, ResUser>(sql_str.as_str())
        .fetch_all(conn)
        .await?;
    Api::success(result)
}