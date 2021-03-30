use serde::{Serialize};
use crate::util::status;
use {tide::Response, tide::prelude::*,tide::StatusCode};


use crate::util::error::{ApiErr, MetaType};
use std::option::Option::Some;
use serde_json::Value;


pub async fn handler(mut res: Response) -> tide::Result {

    // 获取错误信息
    let error = match res.take_error() {
        None => return Ok(res),
        Some(err) => err
    };

    // 特定类型转换
    if let Some(err) = error.downcast_ref::<ApiErr>() {
        tide::log::info!("success!!!");
        let meta: Option<MetaType> = match &err.meta.is_empty() {
            false => Some(err.meta.clone()),
            true => None
        };
        let api = Api::new(None, err.status,  meta);
        res.set_body(api.body());
        res.set_status(StatusCode::Ok);
        return Ok(res);
    }

    tide::log::info!("{:#?}", error);
    // 状态码映射
    match res.status() {
        StatusCode::Ok => Ok(res),
        StatusCode::UnprocessableEntity => ApiErr::status(&status::BAD_REQUEST).build(),
        StatusCode::Unauthorized => ApiErr::status(&status::UNAUTH).build(),
        StatusCode::BadRequest => ApiErr::status(&status::BAD_REQUEST).build(),
        _ => ApiErr::status(&status::SYS_ERROR).build(),
    }
}


#[derive(Serialize)]
pub struct Api<T: Serialize> {
    pub code: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<T>,
}

impl<T: Serialize> Api<T> {
    pub fn new(data: Option<T>, status: &status::Status, err: Option<T>) -> Self {
        Self {
            code: status.0,
            data,
            message: status.1.clone(),
            errors: err,
        }
    }

    pub fn builder(status: &status::Status) -> Self {
        Self::new(None, status, None)
    }

    #[allow(dead_code)]
    pub fn status(&mut self, status: &status::Status) -> &mut Self {
        self.code = status.0;
        self.message = status.1.clone();
        self
    }

    #[allow(dead_code)]
    pub fn message(&mut self,message: String) -> &mut Self{
        self.message = message;
        self
    }

    #[allow(dead_code)]
    pub fn data(&mut self,data: T) -> &mut Self{
        self.data = Some(data);
        self
    }

    #[allow(dead_code)]
    pub fn errors(&mut self,errors: T) -> &mut Self{
        self.errors = Some(errors);
        self
    }

    #[allow(dead_code)]
    pub fn error_validate(errors: T) -> tide::Result {
        Self::builder(&status::BAD_REQUEST).errors(errors).response()
    }

    #[allow(dead_code)]
    pub fn response(&self) -> tide::Result {
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(self.body());
        Ok(res)
    }

    pub fn body(&self) -> Value{
        let s_json = json!(self);
        s_json
    }

    #[allow(dead_code)]
    pub fn success(data: T) -> tide::Result {
        Self::new(Some(data), &status::OK, None).response()
    }

    #[allow(dead_code)]
    pub fn error(e: Option<T>) -> tide::Result {
        Self::new(None, &status::BAD_REQUEST, e).response()
    }

}