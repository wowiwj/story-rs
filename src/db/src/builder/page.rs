use serde::{Deserialize, Serialize};

pub struct Page {
    pub page: usize,
    pub limit: usize,
}

impl Page {
    pub fn default() -> Self {
        Self {
            page: 1,
            limit: 20,
        }
    }

    pub fn new(page: usize, limit: usize) -> Self {
        Self {
            page,
            limit,
        }
    }

    pub fn page(&mut self, page: usize) -> &mut Self {
        self.page = page;
        self
    }

    pub fn limit(&mut self, limit: usize) -> &mut Self {
        self.limit = limit;
        self
    }
}

#[derive(Serialize,Deserialize)]
pub struct PageInfo<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub per_page: usize,
    pub current_page: usize,
    pub total_page: usize,
}

impl<T> PageInfo<T> {
    pub fn new(items: Vec<T>, total: u64, page: Page) -> Self {
        Self {
            items: items,
            total: total,
            per_page: page.limit,
            current_page: page.page,
            total_page: total as usize / page.limit + 1,
        }
    }
}