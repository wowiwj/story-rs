use common::state::State;

use common::api::api::Api;
use db::builder::builder::QueryX;
use quaint::ast::{Select, Comparable};
use db::models::stories::Story;
use db::builder::page::{Page, PageInfo};

pub async fn stories(req: tide::Request<State>) -> tide::Result {
    let page = Page::default();
    let select = Select::from_table("stories").so_that(
        "deleted_at".is_null()
    );
    let conn = &req.state().db;
    let data: PageInfo<Story> = QueryX::page_as(select,page,conn).await?;
    Api::success(data)
}