use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Json, Nothing};
use anyhow::Error;
use serde_json::json;
use js_sys::Date;
use crate::models::game_boards::TootOttoGameBoard;
use std::f64;

extern crate models;
use models::game::Game;

pub struct TootOttoHuman {
    link: ComponentLink<Self>,
    game: Game,
    selected_disc: char,
    game_started: bool,
    context: Option<CanvasRenderingContext2d>,
    board: TootOttoGameBoard,
    move_num: u8,
    won: bool,
    paused: bool,
    save_task: Option<Result<FetchTask, Error>>,
    player1_color: String,
    player2_color: String,
    board_color: String
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {
    GotPlayer1Input(String),
    GotPlayer2Input(String),
    ClickedStart,
    DiscTSelected,
    DiscOSelected,
    ClickedBoard(MouseEvent),
    GetGamesList(Vec<Game>),
    GameSaved,
    SaveError,
    Player1ColorChange(String),
    Player2ColorChange(String),
    BoardColorChange(String)
}

impl Component for TootOttoHuman {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoHuman {
            link,
            game: Game {
                game_number: 0, // placeholder, when game is saved this can be set
                game_type: "TOOT-OTTO".into(),
                player1_name: "".into(),
                player2_name: "".into(),
                winner_name: "".into(),
                game_date: 0 // placeholder, when game is saved this can be set
            },
            selected_disc: 'T',
            game_started: false,
            context: None,
            board: TootOttoGameBoard {
                rows: 6,
                columns: 7,
                tokens: [[0; 7]; 6],
                disc_map: [['0'; 7]; 6]
            },
            move_num: 0,
            won: false,
            paused: false,
            save_task: None,
            player1_color: "#99ffcc".into(),
            player2_color: "#ffff99".into(),
            board_color: "#00bfff".into(),
        }
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotPlayer1Input(new_value) => {
                self.game.player1_name = new_value;
            },
            Msg::GotPlayer2Input(new_value) => {
                self.game.player2_name = new_value;
            },
            Msg::ClickedStart => {
                if self.game.player1_name.is_empty()
                    || self.game.player2_name.is_empty() {
                    //TODO: Show an error message
                } else if self.game.player1_name == "Computer"
                    || self.game.player2_name == "Computer" {

                } else if self.game.player1_name == self.game.player2_name {

                } else {
                    self.game_started = true;
                    self.draw_board();
                    self.print();
                }
            },
            Msg::DiscTSelected => {
                self.selected_disc = 'T';
            },
            Msg::DiscOSelected => {
                self.selected_disc = 'O';
            },
            Msg::ClickedBoard(event) => {
                if !self.game_started {
                    return false;
                }
                if self.won {
                    log!("Resetting board");
                    self.reset();
                    return true; // Reload Html
                }
                let rect = self.canvas().get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                // log!("x: {} y: {}", x,y);

                for i in 0..7 {
                    if self.on_region([x, y], (75 * i + 100) as f64, 25.0){
                        // log!("Region {} clicked", i);
                        self.paused = false;
                        let valid = self.action(i as f64);
                        if valid == 1 {
                            //Reject Click
                        }
                        break; //because there will be no 2 points that are clicked at a time
                    }
                    
                }
            },
            Msg::GetGamesList(games) => {
                log!("Obtained games list");
                self.game.game_number = games.len() as i32 + 1;
                self.save_task = None;
                self.save_game();
            },
            Msg::GameSaved => {
                log!("Successfully saved");
                self.save_task = None;
            },
            Msg::SaveError => log!("Game failed to save"),
            Msg::Player1ColorChange(new_value) => {
                self.player1_color = new_value;
            },
            Msg::Player2ColorChange(new_value) => {
                self.player2_color = new_value;
            },
            Msg::BoardColorChange(new_value) => {
                self.board_color = new_value;
            },
        }
        true
    }
    

    fn mounted(&mut self) -> ShouldRender {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("toot-otto-human-gameboard").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        self.context = Some(canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap());

        false
    }

    fn view(&self) -> Html {
        let game_details;
        if self.game_started {
            game_details = html! {
                <div>
                    <br></br>
                    <h4>{"New Game: "}  {&self.game.player1_name} {" Vs "} {&self.game.player2_name}</h4>
                    <small>{"(Winning Combination: "}  {&self.game.player1_name} {" - TOOT "} {&self.game.player2_name} {" - OTTO)"}</small>
                    <br></br>
                    <form>
                        <h4>{"Select a Disc Type: "}
                            <input
                                id="discT"
                                type="radio"
                                name="choice"
                                value="T"
                                checked=(self.selected_disc == 'T')
                                onclick=self.link.callback(|_| Msg::DiscTSelected) />{" T "}
                            <input
                                id="discO"
                                type="radio"
                                name="choice"
                                value="O"
                                checked=(self.selected_disc == 'O')
                                onclick=self.link.callback(|_| Msg::DiscOSelected) />{" O "}
                        </h4>
                    </form>
                </div>
            }
        } else {
            game_details = html!{}
        }
        html! {
            <>
                <div class="w3-container">
                    <div class="w3-container">
                        <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Player Names"}</b></h5>
                        <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    </div>
                    <form
                        onsubmit=self.link.callback(|_| Msg::ClickedStart)
                        action="javascript:void(0);">
                    
                        <div class="w3-row-padding" style="padding:4px;">
                            <div class="w3-quarter">
                                <label for="nameInput1">{"Player 1 Name:"}</label>
                                <input
                                    class="w3-input w3-border w3-round"
                                    id="nameInput1"
                                    type="text"
                                    disabled=self.game_started
                                    value=&self.game.player1_name
                                    oninput=self.link.callback(|e: InputData| Msg::GotPlayer1Input(e.value))
                                    placeholder="Enter name"
                                />
                            </div>

                            <div class="w3-quarter">
                                <label for="nameInput2">{"Player 2 Name:"}</label>
                                <input
                                    class="w3-input w3-border w3-round"
                                    id="nameInput2"
                                    type="text"
                                    disabled=self.game_started
                                    value=&self.game.player2_name
                                    oninput=self.link.callback(|e: InputData| Msg::GotPlayer2Input(e.value))
                                    placeholder="Enter name"/>
                            </div>
                            
                            <div class="w3-quarter">
                                <label for="player1_color">{"Disc Color Player 1:"}</label>
                                <input
                                    style="display:block;"
                                    oninput=self.link.callback(|e: InputData| Msg::Player1ColorChange(e.value)) 
                                    type="color" 
                                    id="player1_color"
                                    value=&self.player1_color 
                                    disabled=self.game_started
                                />
                            </div>
                            <div class="w3-quarter">
                                <label for="player2_color">{"Disc Color Player 2:"}</label>
                                <input
                                    style="display:block;"
                                    oninput=self.link.callback(|e: InputData| Msg::Player2ColorChange(e.value))
                                    type="color" 
                                    id="player2_color" 
                                    value=&self.player2_color 
                                    disabled=self.game_started
                                />
                            </div>
                        </div>
                        <div class="w3-row-padding" style="padding:4px;">
                            <div class="w3-threequarter">
                                <label for="board_color">{"Board Color:"}</label>  
                                <input
                                    style="display:block;"
                                    oninput=self.link.callback(|e: InputData| Msg::BoardColorChange(e.value))
                                    type="color" 
                                    id="board_color"
                                    name="favcolor"
                                    value=&self.board_color
                                    disabled=self.game_started
                                />
                            </div>  
                        </div>
                        <div class="w3-row-padding" style="padding:4px;">
                            <input
                                class="w3-button w3-border w3-block"
                                id="startButton"
                                disabled=self.game_started
                                type="submit"
                                value="Start Game"
                            />
                        </div>
                    </form>
                </div>

                {game_details}

                <canvas
                    onclick=self.link.callback(|e| Msg::ClickedBoard(e))
                    id="toot-otto-human-gameboard"
                    height="480"
                    width="640">
                </canvas>
            </>
        }
    }
}

impl TootOttoHuman {
    fn draw_board(&self) {
        let context = self.context();

        context.save();
        context.set_fill_style(&JsValue::from_str(&self.board_color));
        context.begin_path();
        for y in 0..6 {
            let y = y as f64;
            for x in 0..7 {
                let x = x as f64;
                context.arc(75.0 * x + 100.0, 75.0 * y + 50.0, 25.0, 0.0, 2.0 * f64::consts::PI).unwrap();
                context.rect(75.0 * x + 150.0, 75.0 * y, -100.0, 100.0);
            }
        }
        context.fill();
        context.restore();
    }

    fn draw(&self){
        let mut fg_color = "transparent".to_string();
        for y in 0..6 {
            for x in 0..7 {
                let mut text = ' ';
                fg_color = "transparent".to_string();
                if self.board.tokens[y][x] >= 1 && self.board.disc_map[y][x] == 'T' {
                    fg_color = self.player1_color.clone();
                    text = 'T';
                } else if self.board.tokens[y][x] >= 1 && self.board.disc_map[y][x] == 'O' {
                    fg_color = self.player1_color.clone();
                    text = 'O';
                } else if self.board.tokens[y][x] <= -1_i8 && self.board.disc_map[y][x] == 'T' {
                    fg_color = self.player2_color.clone();
                    text = 'T';
                } else if self.board.tokens[y][x] <= -1_i8 && self.board.disc_map[y][x] == 'O' {
                    fg_color = self.player2_color.clone();
                    text = 'O';
                }

                self.draw_circle((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, fg_color, "black".to_string(), text.to_string());
            }
        }
    }

    fn draw_circle(&self, x: f64, y: f64, r: f64, fill: String, stroke: String, text: String) {
        let context = self.context();

        context.save();
        context.set_fill_style(&JsValue::from_str(&fill));
        context.set_stroke_style(&JsValue::from_str(&stroke));
        context.begin_path();
        context.arc(x, y, r, 0.0, 2.0 * f64::consts::PI).unwrap();
        context.fill();
        // TODO: set font family??
        context.set_font("bold 25px serif");
        context.restore();
        context.fill_text(&text.as_str(), x - 8.5, y + 8.0);
    }

    fn draw_mask(&self) {
        // draw the mask
        // http://stackoverflow.com/questions/6271419/how-to-fill-the-opposite-shape-on-canvas
        // -->  http://stackoverflow.com/a/11770000/917957

        let context = self.context();
        context.save();
        context.set_fill_style(&JsValue::from_str(&"#00bfff"));
        context.begin_path();
        for y in 0..6 {
            for x in 0..7 {
                context.arc(75.0 * x as f64 + 100.0, 75.0 * y as f64 + 50.0, 25.0, 0.0, 2.0 * f64::consts::PI);
                context.rect(75.0 * x as f64 + 150.0, 75.0 * y as f64, -100.0, 100.0);
            }
        }
        context.fill();
        context.restore();
    }

    fn on_region(&self, coord: [f64; 2], x: f64, radius: f64) -> bool {
        if (coord[0] - x as f64) * (coord[0] - x as f64) <=  radius * radius {
            return true;
        }
        return false;
    }

    fn clear(&self) {
        let context = self.context();
        let canvas = self.canvas();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }

    // fn animate(&self, column: f64, move_num: i8, to_row: f64, cur_pos: f64, callback: js_sys::Function){
    //     let mut fg_color = "transparent";
    //     if move_num >= 1 {
    //         fg_color = "#ff4136";
    //     } else if move_num <= -1 {
    //         fg_color = "#ffff00";
    //     }
    //     if to_row * 75.0 >= cur_pos {
    //         self.clear();
    //         self.draw();
    //         self.draw_circle(75.0 * column + 100.0, cur_pos + 50.0, 25.0, fg_color.to_string(), "black".to_string());
    //         self.draw_mask();
    //         let window = self.window();
    //         let closure = Closure::wrap(Box::new(move || {
    //             Self::animate(self, column, move_num, to_row, cur_pos + 25.0, callback);
    //         }) as Box<dyn Fn()>);
    //         // = Closure::wrap(Box::new(move || self.animate(column, move_num, to_row, cur_pos + 25.0, callback) as Box<dyn Fn()>));
    //         // window.request_animation_frame(function.as_ref().unchecked_ref());
    //     } else {
    //         callback.call0(&JsValue::NULL);
    //     }
    // }

    fn action(&mut self, column: f64) -> i8{
        if self.paused || self.won {
            return 0;
        }
        if self.board.tokens[0][column as usize] != 0 || column < 0.0 || column > 6.0 {
            return -1;
        }

        let mut row = 0;
        let mut done = false;
        for i in 0..5 {
            match self.board.tokens[i + 1][column as usize] {
                0 => continue,
                _=> {
                    done = true;
                    row = i;
                    break;
                }
            }
        }
        if !done {
            row = 5;
        }
        // log!("Adding token to row {}", row);
        self.board.tokens[row as usize][column as usize] = self.player_token();
        self.board.disc_map[row as usize][column as usize] = self.selected_disc;
        self.move_num += 1;
        self.draw();
        self.check();
        self.print();
        // self.animate(column, row as u8);
        
        self.paused = true;
        return 1;
    }

    fn check (&mut self) {
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (['0'; 4], ['0'; 4], ['0'; 4], ['0'; 4]);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = ['0'; 4];
                temp_b = ['0'; 4];
                temp_br = ['0'; 4];
                temp_tr = ['0'; 4];
                for k in 0..=3 {
                    //from (i,j) to right
                    if j + k < 7 {
                        temp_r[k] = self.board.disc_map[i][j + k];
                    }
                    //from (i,j) to bottom
                    if i + k < 6 {
                        temp_b[k] = self.board.disc_map[i + k][j];
                    }

                    //from (i,j) to bottom-right
                    if i + k < 6 && j + k < 7 {
                        temp_br[k] = self.board.disc_map[i + k][j + k];
                    }

                    //from (i,j) to top-right
                    if (i - k) as i8 >= 0 && j + k < 7 {
                        temp_tr[k] = self.board.disc_map[i - k][j + k];
                    }
                }

                if temp_r[0] == 'T' && temp_r[1] == 'O' && temp_r[2] == 'O' && temp_r[3] == 'T' {
                    self.win(1);
                }
                else if temp_r[0] == 'O' && temp_r[1] == 'T' && temp_r[2] == 'T' && temp_r[3] == 'O' {
                    self.win(-1);
                }
                else if temp_b[0] == 'T' && temp_b[1] == 'O' && temp_b[2] == 'O' && temp_b[3] == 'T' {
                    self.win(1);
                }
                else if temp_b[0] == 'O' && temp_b[1] == 'T' && temp_b[2] == 'T' && temp_b[3] == 'O' {
                    self.win(-1);
                }
                else if temp_br[0] == 'T' && temp_br[1] == 'O' && temp_br[2] == 'O' && temp_br[3] == 'T' {
                    self.win(1);
                }
                else if temp_br[0] == 'O' && temp_br[1] == 'T' && temp_br[2] == 'T' && temp_br[3] == 'O' {
                    self.win(-1);
                }
                else if temp_tr[0] == 'T' && temp_tr[1] == 'O' && temp_tr[2] == 'O' && temp_tr[3] == 'T' {
                    self.win(1);
                }
                else if temp_tr[0] == 'O' && temp_tr[1] == 'T' && temp_tr[2] == 'T' && temp_tr[3] == 'O' {
                    self.win(-1);
                }

            }
        }
        // check if draw
        if self.move_num == 42 && !self.won {
            self.win(0);
        }
    }

    fn win(&mut self, player: i8) {
        self.paused = true;
        self.won = true;
        self.game.game_date = Date::new_0().get_time() as i64; // Set date using js_sys::Date (chrono does not seem to work)
        let mut msg = "".to_string();
        if player > 0 {
            msg.push_str(format!("{} wins", self.game.player1_name).as_str());
            self.game.winner_name = self.game.player1_name.clone();
        }
        else if player < 0 {
            msg.push_str(format!("{} wins", self.game.player2_name).as_str());
            self.game.winner_name = self.game.player2_name.clone();
        }
        else{
            msg.push_str("It's a draw");
            self.game.winner_name = "Draw".to_string();
        }
        msg.push_str(" - Click on game board to reset");
        let context = self.context();
        context.save();
        context.set_font("14pt sans-serif");
        context.set_fill_style(&JsValue::from_str("#111"));
        context.fill_text(&msg, 150.0, 20.0);

        // Print final state
        log!("{}", msg);

        // Save game using API
        self.get_games_list();
    }

    // Returns i if it is the player's token, else -1 (for computer)
    fn player_token (&self) -> i8 {
        if self.move_num %2 == 0 {
            return 1;
        } else {
            return -1;
        }
    }

    // Print board, disc map, and move number
    fn print(&self) {
        let (mut msg, mut disc_msg) = ("".to_string(), "".to_string());
        msg.push_str("\n");
        msg.push_str(format!("Move: {}", self.move_num).as_str());
        msg.push_str("\n");
        for i in 0..6 {
            for j in 0..7 {
                msg.push_str(format!(" {}", self.board.tokens[i][j]).as_str());
                disc_msg.push_str(format!(" {}", self.board.disc_map[i][j]).as_str());
            }
            msg.push_str("\n");
            disc_msg.push_str("\n");
        }
        log!("{}", msg);
        log!("{}", disc_msg);
    }

    fn get_games_list(&mut self) {
        // Create GET request for list of games
        let get_request = Request::get("http://localhost:8000/list-games")
            .body(Nothing)
            .unwrap();

        // Create task for FetchService
        let task = FetchService::new().fetch(
            get_request,
            self.link.callback(|response: Response<Json<Result<Vec<Game>, Error>>>| {
                if let (meta, Json(Ok(body))) = response.into_parts() {
                    if meta.status.is_success() {
                        return Msg::GetGamesList(body);
                    }
                }
                Msg::SaveError
            }),
        );

        // Store reference to task
        self.save_task = Some(task);
    }

    fn save_game(&mut self) {
        // Create JSON representation of game to save
        let json_game = json!{self.game};

        // Create POST request to save game
        let post_request = Request::post("http://localhost:8000/insert-game")
            .header("Content-Type", "application/json")
            .body(Json(&json_game))
            .expect("Failed to build request.");

        // Create save task
        let task = FetchService::new().fetch(
            post_request,
            self.link.callback(|response: Response<Result<String, Error>>| {
                log!("In callback function");
                if response.status().is_success() {
                    Msg::GameSaved
                } else {
                    Msg::SaveError
                }
            }),
        );

        // Store reference to task
        self.save_task = Some(task);
    }

    fn reset(&mut self) {
        self.clear();
        self.game = Game {
            game_number: 0, // placeholder, when game is saved this can be set
            game_type: "TOOT-OTTO".into(),
            player1_name: "".into(),
            player2_name: "".into(),
            winner_name: "".into(),
            game_date: 0 // placeholder, when game is saved this can be set
        };
        self.selected_disc = 'T';
        self.game_started = false;
        self.board.tokens = [[0; 7]; 6];
        self.board.disc_map = [['0'; 7]; 6];
        self.move_num = 0;
        self.won = false;
        self.paused = false;
    }

    fn context(&self) -> &CanvasRenderingContext2d {
        self.context.as_ref().unwrap()
    }

    fn canvas(&self) -> HtmlCanvasElement {
        self.context.as_ref().unwrap().canvas().unwrap()
    }

    fn window(&self) -> Window {
        web_sys::window().unwrap()
    }
}
