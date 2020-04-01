#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate bson;
use mongodb::{Client};

mod models;
mod game;

static mut MC: Option<Client> = None;
static DB_NAME: &'static str = "Connect4DB";
static GAMES_TEST_COLLECTION: &'static str = "games-test";

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

fn main() -> Result<(), mongodb::error::Error> {
    unsafe {
        let client = Client::with_uri_str("mongodb://localhost:27017/")?;
        MC = Some(client);
    }

    rocket::ignite()
        .mount("/", routes![
            hello,
            hi,
            game::insert_game_test,
            game::list_games,
            game::insert_default_test
        ])
        .launch();
    Ok(())
}
