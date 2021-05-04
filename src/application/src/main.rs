#[macro_use]
extern crate rocket;

use diesel::PgConnection;
use rocket_contrib::databases::database;
use rocket::{Rocket, Build};


#[database("story_db")]
pub struct DBPool(PgConnection);


#[get("/")]
fn hello() -> String {
    format!("Hello, year old named !")
}

fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello])
}


#[rocket::main]
async fn main() {
    if let Err(e) = rocket().launch().await {
        println!("Whoops! Rocket didn't launch!");
        // We drop the error to get a Rocket-formatted panic.
        drop(e);
    };
}