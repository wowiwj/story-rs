use config::{Config, File, ConfigError};


#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: i32,
    pub domain: String,
}

impl Server {
    pub fn listener(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}



#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub server: Server
}

impl Setting {
    pub fn new(name: &str) -> Result<Setting, ConfigError> {
        let mut s = Config::default();
        let filename = format!("./config/{}", name);
        s.merge(File::with_name(&filename))?;
        s.try_into()
    }
}