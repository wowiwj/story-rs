use config::{Config, File, ConfigError};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u32,
    pub domain: String,
}

impl Server {
    pub fn listener(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum DatabaseDriver {
    #[serde(rename = "mysql")]
    Mysql,
    #[serde(rename = "sqlite")]
    Sqlite,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Database {
    pub driver: DatabaseDriver,
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u32,
    pub database: String,
}

impl Database {
    pub fn url(&self) -> String {
        match self.driver {
            DatabaseDriver::Mysql => {
                format!("mysql://{}:{}@{}:{}/{}", self.username, self.password, self.host, self.port, self.database)
            }
            DatabaseDriver::Sqlite => {
                format!("sqlite::memory:")
            }
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub server: Server,
    pub database: Database,
}


impl Setting {
    pub fn new(name: &str) -> Result<Setting, ConfigError> {
        let mut s = Config::default();
        let filename = format!("./config/{}", name);
        s.merge(File::with_name(&filename))?;
        s.try_into()
    }
}