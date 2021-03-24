use chrono::prelude::{DateTime};
use serde::{Deserialize, Serialize};
use chrono::{Utc};
use crate::util::date_format;

#[derive(Deserialize, Serialize)]
pub(crate) struct ResUser {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    #[serde(with = "date_format")]
    pub created_at: DateTime<Utc>,
}