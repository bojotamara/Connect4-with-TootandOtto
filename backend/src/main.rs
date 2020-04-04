#![feature(decl_macro)]
#![feature(proc_macro_hygiene)]
use rocket_cors;

#[macro_use] extern crate rocket;
#[macro_use] extern crate bson;
extern crate models;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use mongodb::{Client};

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

    // TODO: Error handling for to_cors()
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8080"]); // Set origin to that of app
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::ignite()
        .mount("/", routes![
            hello,
            hi,
            game::insert_game_test,
            game::list_games,
            game::insert_default_test
        ])
        .attach(cors)
        .launch();
    Ok(())
}
