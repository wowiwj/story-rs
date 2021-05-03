use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, MySqlPool};
use serde::Serialize;
use quaint::prelude::*;
use crate::builder::builder::QueryX;
use common::jwt::jwt::AuthUser;


#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub enum Gender {
    None = 0,
    Male = 1,
    Female = 2,
}

#[derive(FromRow, Serialize, Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub gender: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn create(&self, pool: &MySqlPool) -> anyhow::Result<u64> {
        let mut conn = pool.acquire().await?;
        let id = sqlx::query!(r#"
        insert into users
        (name,email,phone,password,gender,created_at,updated_at)
        values (?,?,?,?,?,?,?) "#,
           self.name,
           self.email,
           self.phone,
           self.password,
           self.gender,
           self.created_at,
           self.updated_at
        )
            .execute(&mut conn)
            .await?
            .last_insert_id();
        Ok(id)
    }


    pub async fn find_by_email(email: &str, pool: &MySqlPool) -> anyhow::Result<User> {
        let select = Select::from_table("users").so_that(
            "email".equals(email)
                .and("deleted_at".is_null())
        );

        let u: User = QueryX::first_as(select, pool).await?;
        return Ok(u);
    }
}

impl From<&User> for AuthUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.name.to_string(),
            email: user.email.to_string(),
        }
    }
}