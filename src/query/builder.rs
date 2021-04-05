use sqlx::mysql::{MySqlArguments, MySqlRow};
use quaint::prelude::*;
use sqlx::{FromRow, Arguments, MySqlPool,MySql as MysqlX};
use quaint::visitor::{Mysql, Visitor};
use sqlx::query::QueryAs;

pub struct QueryX;


impl QueryX {}

impl QueryX {
    pub async fn first_as<'a, Q, T>(query: Q, pool: &MySqlPool) -> anyhow::Result<T>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin,
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        let params = Self::arguments(params);
        let query: QueryAs<MysqlX, T, MySqlArguments> = sqlx::query_as_with(sql.as_str(), params);
        let u = query.fetch_one(&mut conn).await?;
        Ok(u)
    }

    pub fn arguments(params: Vec<Value>) -> MySqlArguments {
        let mut arguments = MySqlArguments::default();
        params.into_iter().for_each(|param| {
            &arguments.add(param.as_str());
        });
        arguments
    }
}


