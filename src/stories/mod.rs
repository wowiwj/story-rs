mod handlers;

use tide::Server;
use common::state::State;

pub fn routes(router: &mut Server<State>) {
    let mut stories = router.at("stories");

    stories.at("/").get(handlers::stories);
}