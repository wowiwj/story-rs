use serde::{Serialize};
use crate::util::status;
use {tide::Response, tide::prelude::*,tide::StatusCode};

use std::collections::HashMap;



#[derive(Serialize)]
pub struct ApiErr {
    pub meta: HashMap<String, Vec<String>>
}

impl ApiErr {

    #[allow(dead_code)]
    pub fn builder() -> Self {
        Self {
            meta: HashMap::new()
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, typ: &str, info: &str) -> &mut Self {
        let meta_info = self.meta.get_mut(typ);
        match meta_info {
            Some(meta) => {
                meta.push(String::from(info));
                self
            }
            None => {
                let meta = vec![String::from(info)];
                self.meta.insert(String::from(typ), meta);
                self
            }
        }
    }

    #[allow(dead_code)]
    pub fn build(&self) -> tide::Result {
        Api::error(self.meta.clone())
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
        let s_json = json!(self);
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(s_json);
        Ok(res)
    }

    #[allow(dead_code)]
    pub fn success(data: T) -> tide::Result {
        Self::new(Some(data), &status::OK, None).response()
    }

    #[allow(dead_code)]
    pub fn error(e: T) -> tide::Result {
        Self::new(None, &status::BAD_REQUEST, Some(e)).response()
    }

}