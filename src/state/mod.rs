use sqlx::{MySqlPool};
use crate::CONFIG;
use sqlx::mysql::{MySqlConnectOptions};


#[derive(Clone)]
pub struct State {
    pub db: MySqlPool
}

impl State {
    pub async fn new() -> tide::Result<Self> {
        tide::log::info!("{}", &CONFIG.database.url());
        // let db = MySqlPool::connect(&CONFIG.database.url()).await?;
        let conf = &CONFIG.database;
        let options = MySqlConnectOptions::new()
            .database(&conf.database)
            .port(conf.port)
            .host(&conf.host)
            .username(&conf.username)
            .password(&conf.password);

        let db = MySqlPool::connect_with(options).await?;
        Ok(State {
            db
        })
    }
}