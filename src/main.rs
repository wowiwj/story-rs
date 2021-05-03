#![feature(in_band_lifetimes)]
#![feature(allocator_api)]
#[macro_use]
extern crate lazy_static;

use tide::log;
use tide::utils::After;

use common::state::State;
use common::setting;

mod users;
mod util;
mod models;
mod builder;
mod stories;


lazy_static!(
    static ref CONFIG: setting::Setting = setting::Setting::new("config").expect("Config Load Error");
);

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    log::info!("setting, {}", CONFIG.server.domain);
    let state = State::new(&CONFIG).await?;
    let mut app = tide::with_state(state.clone());

    app.with(After(common::api::api::handler));

    app.at("/api").nest({
        let mut api = tide::with_state(state.clone());
        users::routes(&mut api);
        stories::routes(&mut api);
        api
    });

    app.at("/").get(|_| async {
        Ok("hello world")
    });
    log::info!("app is running");
    app.listen(CONFIG.server.clone().listener()).await?;
    Ok(())
}