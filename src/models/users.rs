use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{FromRow, MySqlPool};
use crate::users::schema::Register;
use serde::Serialize;
use crate::util::crypt::hash_password;
use quaint::prelude::*;
use crate::builder::builder::QueryX;


#[derive(sqlx::Type, Debug)]
#[sqlx(rename_all = "lowercase")]
pub(crate) enum Gender {
    None = 0,
    Male = 1,
    Female = 2,
}

#[derive(FromRow, Serialize, Clone)]
pub(crate) struct User {
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

impl From<Register> for User {
    fn from(r: Register) -> Self {
        let now = Utc::now().into();
        User {
            id: 0,
            name: r.username,
            email: r.email,
            phone: r.phone,
            password: hash_password(&r.password),
            gender: Gender::None as u32,
            created_at: now,
            updated_at: None,
            deleted_at: None,
        }
    }
}