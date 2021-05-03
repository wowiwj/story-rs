
use common::state::State;

use crate::users::schema::{ResUser, Register, ResAuthUser, Login};
use validator::Validate;
use common::api::api::{Api};
use db::models::users::User;
use std::result::Result::Err;
use common::api::error::ApiErr;

use quaint::ast::Select;
use quaint::prelude::*;
use db::builder::builder::QueryX;
use common::jwt::jwt::AuthUser;
use common::hash::crypt::password_verify;
use common::jwt::auth::Auth;


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
        .so_that("deleted_at".is_null());
    let result: Vec<ResUser> = QueryX::find_as(select, conn).await?;
    Api::success(result)
}