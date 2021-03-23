use chrono::prelude::{DateTime};
use serde::{Deserialize, Serialize};
use chrono::{Utc};

#[derive(Deserialize, Serialize)]
pub(crate) struct ResUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone:Option<String>,
    pub created_at: Option<DateTime<Utc>>
}