use serde::{Serialize, Deserialize};
// use bson::oid::ObjectId;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct BsonGame {
//     #[serde(rename = "_id")]
//     pub id: ObjectId,
//     pub game_number: i32,
//     pub game_type: String,
//     pub player1_name: String,
//     pub player2_name: String,
//     pub winner_name: String,
//     pub game_date: i64
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub game_number: i32,
    pub game_type: String,
    pub player1_name: String,
    pub player2_name: String,
    pub winner_name: String,
    pub game_date: i64
}

impl Game {
    pub fn to_json_string(&self) -> String {
        format!(r#"{{"game_number": {}, "game_type": "{}", "player1_name": "{}", "player2_name": "{}", "winner_name": "{}", "game_date": {} }}"#,
                self.game_number, self.game_type, self.player1_name, self.player2_name, self.winner_name, self.game_date)
    }
}

// impl Game {
//     pub fn set_game_number(&mut self, number: i32) {
//         self.game_number = number;
//     }

//     pub fn get_game_number(&self) -> i32 {
//         self.game_number
//     }

//     pub fn set_game_type(&mut self, game_type: String) {
//         self.game_type = game_type;
//     }

//     pub fn get_game_type(&self) -> String {
//         self.game_type.clone()
//     }

//     pub fn set_player1_name(&mut self, name: String) {
//         self.player1_name = name;
//     }

//     pub fn get_player1_name(&self) -> String {
//         self.player1_name.clone()
//     }

//     pub fn set_player2_name(&mut self, name: String) {
//         self.player2_name = name;
//     }

//     pub fn get_player2_name(&self) -> String {
//         self.player2_name.clone()
//     }

//     pub fn set_winner_name(&mut self, name: String) {
//         self.winner_name = name;
//     }

//     pub fn get_winner_name(&self) -> String {
//         self.winner_name.clone()
//     }

//     pub fn set_game_date(&mut self, date: DateTime<Utc>) {
//         self.game_date = date;
//     }

//     pub fn get_game_date(&self) -> DateTime<Utc> {
//         self.game_date
//     }
// }