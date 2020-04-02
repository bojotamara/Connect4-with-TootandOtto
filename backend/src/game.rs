use bson::doc;
use chrono::Utc;
use models::game::Game;
use rocket::response::content;
use rocket_contrib::json;

use super::{MC, DB_NAME, GAMES_TEST_COLLECTION};

/** 
 *  Insert game into DB using JSON data obtained POST request body
 *  Read: https://rocket.rs/v0.4/guide/requests/#json
 */
#[post("/insert-game-test", format = "application/json", data = "<game>")]
pub fn insert_game_test(game: json::Json<Game>) -> String {
    unsafe {
        match MC {
            Some(ref client) => {
                let db = client.database(DB_NAME);
                let collection = db.collection(GAMES_TEST_COLLECTION);

                let serialized_game = bson::to_bson(&game.0).unwrap();

                if let bson::Bson::Document(document) = serialized_game {
                    let _result = collection.insert_one(document, None);  // Insert into a MongoDB collection
                } else {
                    String::from("Error converting the BSON object into a MongoDB document");
                }

                String::from("Successfully inserted test game!")
            },
            None => String::from("Error"),
        }
    }
}

/** 
 *  Returns list of games in DB as a JSON object array
 */
#[get("/list-games")]
pub fn list_games() -> content::Json<String> {
    unsafe {
        match MC {
            Some(ref client) => {
                let db = client.database(DB_NAME);
                let collection = db.collection(GAMES_TEST_COLLECTION);

                // Don't specify any filters to get all the games
                let cursor = collection.find(None, None).unwrap();

                // Create a string vector
                let mut games_string_vec = Vec::<String>::new();

                for result in cursor {
                    match result {
                        Ok(document) => {
                            games_string_vec.push(bson::Bson::Document(document).to_string());
                        },
                        Err(_e) => (),
                    }
                }

                content::Json(format!("[{}]", games_string_vec.join(", ")))
            },
            None => content::Json(String::from("[]")),
        }
    }
}

/** 
 *  Simple GET request to insert default game data into DB
 */
#[get("/insert-default-test")]
pub fn insert_default_test() -> String {
    unsafe {
        match MC {
            Some(ref client) => {
                let db = client.database(DB_NAME);
                let collection = db.collection(GAMES_TEST_COLLECTION);

                let now = Utc::now();
                let default_game = Game {
                    game_number: 0,
                    game_type: "Connect-4".to_string(),
                    player1_name: "Player 1".to_string(),
                    player2_name: "Player 2".to_string(),
                    winner_name: "Player 1".to_string(),
                    game_date: now.timestamp_millis()
                };

                let serialized_game = bson::to_bson(&default_game).unwrap();

                if let bson::Bson::Document(document) = serialized_game {
                    let _result = collection.insert_one(document, None);  // Insert into a MongoDB collection
                } else {
                    String::from("Error converting the BSON object into a MongoDB document");
                }

                String::from("Successfully inserted test game!")
            },
            None => String::from("Error"),
        }
    }
}