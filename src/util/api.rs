use serde::Serialize;
use crate::util::status;
use {tide::Response, tide::prelude::*,tide::StatusCode};


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

    pub fn status(&mut self, status: &status::Status) -> &mut Self {
        self.code = status.0;
        self.message = status.1.clone();
        self
    }

    pub fn message(&mut self,message: String) -> &mut Self{
        self.message = message;
        self
    }

    pub fn data(&mut self,data: T) -> &mut Self{
        self.data = Some(data);
        self
    }

    pub fn errors(&mut self,errors: T) -> &mut Self{
        self.errors = Some(errors);
        self
    }

    pub fn response(&self) -> tide::Result {
        let s_json = json!(self);
        let mut res = Response::new(StatusCode::Ok);
        res.set_body(s_json);
        Ok(res)
    }

    pub fn success(data: Option<T>) -> tide::Result {
        Self::new(data, &status::OK, None).response()
    }

    pub fn error(e: Option<T>) -> tide::Result {
        Self::new(None, &status::BAD_REQUEST, e).response()
    }
}