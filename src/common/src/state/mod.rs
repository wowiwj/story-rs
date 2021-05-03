use sqlx::{MySqlPool};
use sqlx::mysql::{MySqlConnectOptions};

use crate::setting::Setting;


#[derive(Clone)]
pub struct State {
    pub conf: Setting,
    pub db: MySqlPool,
}

impl State {
    pub async fn new(config: &Setting) -> tide::Result<Self> {
        let conf = &config.database;
        let options = MySqlConnectOptions::new()
            .database(&conf.database)
            .port(conf.port)
            .host(&conf.host)
            .username(&conf.username)
            .password(&conf.password);

        let db = MySqlPool::connect_with(options).await?;
        Ok(State {
            db,
            conf: config.clone(),
        })
    }

    pub fn jwt_secret(&self) -> String {
        String::from(&self.conf.server.jwt_secret)
    }
}