use chrono::prelude::{DateTime};
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use crate::util::date_format;
use validator::Validate;
use crate::models::users::User;

#[derive(Deserialize,Serialize,Validate)]
pub(crate) struct Register {
    pub username: String,
    #[validate(email(message = "请输入正确的邮箱"))]
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub(crate) struct ResUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
}

impl From<User> for ResUser {
    fn from(u: User) -> Self {
        Self {
            id: u.id,
            name: u.name,
            email: u.email,
            phone: u.phone,
            created_at: u.created_at,
        }
    }
}