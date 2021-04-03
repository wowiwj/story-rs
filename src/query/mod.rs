mod select;
mod insert;

use std::option::Option::Some;
use crate::query::select::Select;
use crate::query::insert::Insert;

pub struct QueryBuilder {}


impl QueryBuilder {
    pub fn select(table: &str) -> Select {
        Select::new(table)
    }

    pub fn insert(table: &str) -> Insert {
        Insert::new(table)
    }
}

pub trait FilterAble {
    fn filter(&mut self, filter: &str) -> &mut Self;
    fn build_filters(&self) -> String;
}

#[derive(Debug)]
pub struct Filter {
    filters: Option<Vec<String>>,
}

impl Filter  {
    pub fn new() -> Self {
        Self {
            filters: None
        }
    }
}

impl FilterAble for Filter {
    fn filter(&mut self, filter: &str) -> &mut Self {
        if self.filters.is_none() {
            self.filters = Some(Vec::new());
        }

        self.filters.as_mut().and_then(|filters|
            Some(filters.push(filter.into()))
        );
        self
    }

    fn build_filters(&self) -> String {
        let mut wheres = "".into();
        if let Some(filters) = self.filters.as_ref() {
            wheres += format!(" where {}", filters.join(" and ")).as_str();
        }
        wheres
    }
}





