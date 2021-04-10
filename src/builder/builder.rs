use sqlx::mysql::{MySqlArguments, MySqlRow};
use quaint::prelude::*;
use sqlx::{FromRow, MySqlPool, MySql as MysqlX, Row};
use quaint::visitor::{Mysql, Visitor};
use sqlx::query::QueryAs;
use crate::builder::convert::QueryXArgument;
use crate::builder::page::{Page, PageInfo};


pub struct QueryX;

impl QueryX {
    pub async fn page_as<'a, T>(select: Select<'a>, page: Page, pool: &MySqlPool) -> anyhow::Result<PageInfo<T>>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin {
        let count_select: Select = select.clone().value(count(asterisk()));
        let total: (i64, ) = Self::first_as(count_select, pool)
            .await
            .map_or_else(|_| {
                (0 as i64, )
            }, |res| {
                res
            });
        let select = select.clone().offset(page.page - 1).limit(page.limit);

        let items: Vec<T> = Self::find_as(select, pool).await?;
        Ok(PageInfo::new(items, total.0 as u64 , page))
    }

    pub async fn find_as<'a, Q, T>(query: Q, pool: &MySqlPool) -> anyhow::Result<Vec<T>>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin,
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        let query = Self::bind_as(sql.as_str(), params);
        let u = query.fetch_all(&mut conn).await?;
        Ok(u)
    }

    pub async fn first_as<'a, Q, T>(query: Q, pool: &MySqlPool) -> anyhow::Result<T>
        where
            T: for<'r> FromRow<'r, MySqlRow> + 'a + Send + Unpin,
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        tide::log::info!("{:#?},{:#?}", sql, params);
        let query = Self::bind_as(sql.as_str(), params);
        let u = query.fetch_one(&mut conn).await?;
        Ok(u)
    }

    pub async fn first<'a, Q>(query: Q, pool: &MySqlPool) -> anyhow::Result<MySqlRow>
        where
            Q: Into<Query<'a>> {
        let mut conn = pool.acquire().await?;
        let (sql, params) = Mysql::build(query)?;
        let query = Self::bind(sql.as_str(), params);
        let u = query.fetch_one(&mut conn).await?;
        Ok(u)
    }

    fn bind_as<'a, T>(sql: &'a str, params: Vec<Value<'a>>) -> QueryAs<'a, MysqlX, T, MySqlArguments>
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

    fn bind<'a>(sql: &'a str, params: Vec<Value<'a>>) -> sqlx::query::Query<'a, MysqlX, MySqlArguments>
    {
        if params.is_empty() {
            let query = sqlx::query(sql);
            return query;
        }
        let params = QueryXArgument::new().arguments(params);
        let query = sqlx::query_with(sql, params);
        query
    }
}