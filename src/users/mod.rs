mod handlers;
pub mod schema;

use tide::Server;
use crate::state::State;

pub fn routes(router:&mut Server<State>) {

    // 授权路由
    let mut auth = router.at("/auth");
    auth.at("/register").post(handlers::register);
    auth.at("/login").post(handlers::login);

    // 用户路由
    let mut users = router.at("/users");
    users.at("/").get(handlers::index);
}