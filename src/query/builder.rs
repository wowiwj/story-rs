use sqlx::mysql::{MySqlArguments, MySqlRow};
use quaint::prelude::*;
use sqlx::{FromRow, MySqlPool, MySql as MysqlX};
use quaint::visitor::{Mysql, Visitor};
use sqlx::query::QueryAs;
use crate::query::convert::QueryXArgument;

pub struct QueryX;


impl QueryX {}

impl QueryX {
    pub async fn find_as<'a, Q, T>(query: Q, pool: &MySqlPool) -> anyhow::Result<Vec<T>>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin,
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        let query = Self::bind_query(sql.as_str(), params);
        let u = query.fetch_all(&mut conn).await?;
        Ok(u)
    }

    pub async fn first_as<'a, Q, T>(query: Q, pool: &MySqlPool) -> anyhow::Result<T>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin,
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        let query = Self::bind_query(sql.as_str(), params);
        let u = query.fetch_one(&mut conn).await?;
        Ok(u)
    }

    fn bind_query<'a, T>(sql: &'a str, params: Vec<Value<'a>>) -> QueryAs<'a, MysqlX, T, MySqlArguments>
        where
            T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        if params.is_empty() {
            let query: QueryAs<MysqlX, T, MySqlArguments> = sqlx::query_as(sql);
            return query;
        }
        let params = QueryXArgument::new().arguments(params);
        let query: QueryAs<MysqlX, T, MySqlArguments> = sqlx::query_as_with(sql, params);
        query
    }
}


