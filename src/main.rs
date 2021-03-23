#[macro_use]
extern crate lazy_static;

mod setting;
mod state;
mod users;

use tide::{log,prelude::*};
use crate::state::State;

lazy_static!(
    static ref CONFIG: setting::Setting = setting::Setting::new("config").expect("Config Load Error");
);

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    log::info!("setting, {}", CONFIG.server.domain);
    let state = State::new().await?;
    let mut app = tide::with_state(state.clone());

    app.at("/api").nest({
        let mut api = tide::with_state(state.clone());
        users::routes(&mut api);
        api
    });

    app.at("/").get(|_| async {
        Ok(json!{CONFIG.clone()})
    });
    log::info!("app is running");
    app.listen(CONFIG.server.clone().listener()).await?;
    Ok(())
}