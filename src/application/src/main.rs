#![feature(in_band_lifetimes)]
#![feature(allocator_api)]

use tide::log;
use tide::utils::After;


use common::state::State;
use web::users;
use web::stories;
use common::setting::Setting;


#[async_std::main]
async fn main() -> tide::Result<()> {
    let conf: Setting = Setting::new("config").expect("Config Load Error");
    log::start();
    log::info!("setting, {}", &conf.server.domain);

    let state = State::new(&conf).await?;
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
    app.listen(&conf.server.clone().listener()).await?;
    Ok(())
}