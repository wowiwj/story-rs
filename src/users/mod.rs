mod handlers;

use tide::Server;
use crate::state::State;

pub fn routes(router:&mut Server<State>) {
    let mut users = router.at("/users");
    users.at("/").get(handlers::index);
}