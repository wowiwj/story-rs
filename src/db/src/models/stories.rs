use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow};
use serde::{Serialize};


#[derive(FromRow,Serialize)]
pub struct Story {
    pub id: u64,
    pub title: String,
    pub summary: String,
    pub cover_url: Option<String>,
    pub state: u8,
    pub is_secret: bool,
    pub parent_id: u64,
    pub channel_id: u64,
    pub user_id: u64,
    pub desktop_url: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}