use sqlx::MySqlPool;
use crate::CONFIG;

#[derive(Clone)]
pub struct State {
    db: MySqlPool
}

impl State {
    pub async fn new() -> tide::Result<Self> {
        tide::log::info!("{}",&CONFIG.database.url());
        let db = MySqlPool::connect(&CONFIG.database.url()).await?;
        Ok(State {
            db
        })
    }
}