#[macro_use]
extern crate lazy_static;

mod setting;
mod state;

use tide::{log,prelude::*};

lazy_static!(
    static ref CONFIG: setting::Setting = setting::Setting::new("config").expect("Config Load Error");
);

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    log::info!("setting, {}", CONFIG.server.domain);
    let mut app = tide::new();
    app.at("/").get(|_| async {
        Ok(json!{CONFIG.clone()})
    });
    log::info!("app is running");
    app.listen(CONFIG.server.clone().listener()).await?;
    Ok(())
}