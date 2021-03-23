use tide::Response;
use crate::state::State;

pub async fn index(req: tide::Request<State>) -> tide::Result {
    Ok(Response::from("hello"))
}