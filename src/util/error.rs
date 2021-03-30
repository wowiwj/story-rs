use std::error;
use std::fmt;
use std::collections::HashMap;
use crate::util::status;
use crate::util::api::Api;


pub type MetaType = HashMap<String, Vec<String>>;

#[derive(Debug)]
pub struct ApiErr {
    pub meta: MetaType,
    pub message: String,
    pub status: &'static status::Status,
}



impl fmt::Display for ApiErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "internal server error")
    }
}

impl error::Error for ApiErr {}

impl ApiErr {

    pub fn status(status: &'static status::Status) -> Self {
        Self {
            meta: HashMap::new(),
            message: status.1.to_string(),
            status: status,
        }
    }

    #[allow(dead_code)]
    pub fn builder() -> Self {
        Self {
            meta: HashMap::new(),
            message: String::new(),
            status: &status::UNKNOWN,
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
        if self.meta.is_empty() {
            return  Api::<MetaType>::error(None)
        }
        Api::error(Some(self.meta.clone()))
    }
}