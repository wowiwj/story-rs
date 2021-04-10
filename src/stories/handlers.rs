use crate::state::State;

use crate::util::api::Api;
use crate::builder::builder::QueryX;
use crate::builder::page::{Page, PageInfo};
use quaint::ast::{Select, Comparable};
use crate::models::stories::Story;

pub async fn stories(req: tide::Request<State>) -> tide::Result {
    let page = Page::default();
    let select = Select::from_table("stories").so_that(
        "deleted_at".is_null()
    );
    let conn = &req.state().db;
    let data: PageInfo<Story> = QueryX::page_as(select,page,conn).await?;
    Api::success(data)
}