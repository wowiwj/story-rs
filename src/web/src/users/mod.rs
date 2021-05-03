mod handlers;
pub mod schema;

use tide::Server;
use common::state::State;
use common::jwt::auth::JwtMiddleWare;

pub fn routes(router:&mut Server<State>) {

    // 授权路由
    let mut auth = router.at("/auth");
    auth.at("/register").post(handlers::register);
    auth.at("/login").post(handlers::login);

    // 用户路由
    let mut users = router.with(JwtMiddleWare::new()).at("/users");
    users.at("/").get(handlers::index);
}