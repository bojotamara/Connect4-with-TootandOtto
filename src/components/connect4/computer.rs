use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Json, Nothing};
use anyhow::Error;
use serde_json::json;
use js_sys::{Date, Math};
use crate::models::game_boards::Connect4GameBoard;
use std::f64;
use std::cmp::{max, min};

extern crate models;
use models::game::Game;

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    game: Game,
    game_started: bool,
    context: Option<CanvasRenderingContext2d>,
    board: Connect4GameBoard,
    move_num: u8,
    won: bool,
    paused: bool,
    save_task: Option<Result<FetchTask, Error>>
}

pub struct GameBoard {
    rows: u8,
    columns: u8,
    tokens: [[i8; 7];6]
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
    GotInput(String),
    ClickedStart,
    ClickedBoard(MouseEvent),
    GetGamesList(Vec<Game>),
    GameSaved,
    SaveError
}

impl Component for Connect4Computer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Connect4Computer {
            link,
            game: Game {
                game_number: 0, // placeholder, when game is saved this can be set
                game_type: "Connect4".into(),
                player1_name: "".into(),
                player2_name: "Computer".into(),
                winner_name: "".into(),
                game_date: 0 // placeholder, when game is saved this can be set
            },
            game_started: false,
            context: None,
            board: Connect4GameBoard {
                rows: 6,
                columns: 7,
                tokens: [[0; 7]; 6],
            },
            move_num: 0,
            won: false,
            paused: false,
            save_task: None
        }
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.game.player1_name = new_value;
            },
            Msg::ClickedStart => {
                if self.game.player1_name.is_empty() {
                    //TODO: Show an error message
                } else {
                    self.game_started = true;
                    self.draw_board();
                    self.print();
                }
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
                        let valid = self.action(i);
                        if valid == 1 {
                            // Perform AI action
                            self.paused = false;
                            self.ai();
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
        }
        true
    }
    

    fn mounted(&mut self) -> ShouldRender {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("connect4-comp-gameboard").unwrap();
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
                    <small>{"Disc Colors: "}  {&self.game.player1_name} {" - Red "} {&self.game.player2_name} {" - Yellow "}</small>
                    <br></br>
                </div>
            }
        } else {
            game_details = html!{}
        }
        html! {
            <>
                <div>
                    <div class="w3-container">
                        <h5 class="w3-xxxlarge w3-text-red"><b>{"Enter Your Name"}</b></h5>
                        <hr style="width:50px;border:5px solid red" class="w3-round"/>
                    </div>
                    <div class="col-md-offset-4 col-md-8">
                        <div class="col-md-offset-3 col-md-8">
                            <form
                                onsubmit=self.link.callback(|_| Msg::ClickedStart)
                                action="javascript:void(0);">
                                <input
                                    id="nameInput"
                                    type="text"
                                    disabled=self.game_started
                                    value=&self.game.player1_name
                                    oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                                    placeholder="Your Name"/>
                                <input
                                    id="startButton"
                                    disabled=self.game_started
                                    class="w3-button w3-border"
                                    type="submit"
                                    value="Start Game"/>
                            </form>
                        </div>
                    </div>
                </div>

                {game_details}

                <canvas
                    onclick=self.link.callback(|e| Msg::ClickedBoard(e))
                    id="connect4-comp-gameboard"
                    height="480"
                    width="640">
                </canvas>
                
            </>
        }
    }
}

impl Connect4Computer {
    fn draw_board(&self) {
        let context = self.context();

        context.save();
        context.set_fill_style(&JsValue::from_str("#00bfff"));
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

    fn draw(&self) {
        let mut fg_color = "transparent";
        for y in 0..6 {
            for x in 0..7 {
                fg_color = "transparent";
                if self.board.tokens[y][x] >= 1 {
                    fg_color = "#ff4136";
                } else if self.board.tokens[y][x] <= -1_i8 {
                    fg_color = "#ffff00";
                }
                self.draw_circle((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, fg_color.to_string(), "black".to_string());
            }
        }
    }

    fn draw_circle(&self, x: f64,y: f64, r: f64, fill: String, stroke: String) {
        let context = self.context();

        context.save();
        context.set_fill_style(&JsValue::from_str(&fill));
        context.set_stroke_style(&JsValue::from_str(&stroke));
        context.begin_path();
        context.arc(x, y, r, 0.0, 2.0 * f64::consts::PI).unwrap();
        //this.context.stroke();
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

    // fn animate(&self, column: f64, moveM: u8, row: f64, cur_pos: f64){

    // }

    fn action(&mut self, column: i64) -> i8{
        if self.paused || self.won {
            return 0;
        }
        if self.board.tokens[0][column as usize] != 0 || column < 0 || column > 6 {
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
        self.move_num += 1;
        self.draw();
        self.check();
        self.print();
        // self.animate(column, self.moveNum, row as f64, 0.0);
        
        // Set pause to true to do AI move
        self.paused = true;
        return 1;
    }

    fn check(&mut self){
        let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
        for i in 0..6 {
            for j in 0..7 {
                temp_r = 0;
                temp_b = 0;
                temp_br = 0;
                temp_tr = 0;
                for k in 0..=3 {
                    //from (i,j) to right
                    if j + k < 7 {
                        temp_r += self.board.tokens[i][j + k];
                    }
                    //from (i,j) to bottom
                    if i + k < 6 {
                        temp_b += self.board.tokens[i + k][j];
                    }

                    //from (i,j) to bottom-right
                    if i + k < 6 && j + k < 7 {
                        temp_br += self.board.tokens[i + k][j + k];
                    }

                    //from (i,j) to top-right
                    if (i - k) as i8 >= 0 && j + k < 7 {
                        temp_tr += self.board.tokens[i - k][j + k];
                    }
                }
                if temp_r.abs() == 4 {
                    self.win(temp_r);
                } 
                else if temp_b.abs() == 4 {
                    self.win(temp_b);
                } 
                else if temp_br.abs() == 4 {
                    self.win(temp_br);
                } 
                else if temp_tr.abs() == 4 {
                    self.win(temp_tr);
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

    // Print board and move number
    fn print(&self) {
        let mut msg = "".to_string();
        msg.push_str("\n");
        msg.push_str(format!("Move: {}", self.move_num).as_str());
        msg.push_str("\n");
        for i in 0..6 {
            for j in 0..7 {
                msg.push_str(format!(" {}", self.board.tokens[i][j]).as_str());
            }
            msg.push_str("\n");
        }
        log!("{}", msg);
    }

    fn ai(&mut self) {
        let curr_state = self.board.tokens.clone();

        // On success, returns an array of size 2 -> index 0 gives filled map with char 'T', index 1 gives filled map with char 'O'
        fn fill_map(state: [[i8; 7]; 6], column: i64, value: i8) -> Result<[[i8; 7]; 6], i8> {
            if state[0][column as usize] != 0 || column < 0 || column > 6 {
                return Err(-1);
            }
    
            let mut done = false;
            let mut row = 0;
            for i in 0..5 {
                if state[i + 1][column as usize] != 0 {
                    done = true;
                    row = i;
                    break;
                }
            }
    
            if !done {
                row = 5;
            }
    
            let mut temp_map = state.clone();
            temp_map[row][column as usize] = value;
            return Ok(temp_map);
        }

        fn check_state(state: [[i8; 7]; 6]) -> [i64; 2] {
            let mut win_val = 0;
            let mut chain_val = 0;
            let (mut temp_r, mut temp_b, mut temp_br, mut temp_tr) = (0, 0, 0, 0);
            for i in 0..6 {
                for j in 0..7 {
                    temp_r = 0;
                    temp_b = 0;
                    temp_br = 0;
                    temp_tr = 0;
                    for k in 0..=3 {
                        //from (i,j) to right
                        if j + k < 7 {
                            temp_r += state[i][j + k];
                        }
                        //from (i,j) to bottom
                        if i + k < 6 {
                            temp_b += state[i + k][j];
                        }

                        //from (i,j) to bottom-right
                        if i + k < 6 && j + k < 7 {
                            temp_br += state[i + k][j + k];
                        }

                        //from (i,j) to top-right
                        if (i - k) as i8 >= 0 && j + k < 7 {
                            temp_tr += state[i - k][j + k];
                        }
                    }

                    chain_val += temp_r * temp_r * temp_r;
                    chain_val += temp_b * temp_b * temp_b;
                    chain_val += temp_br * temp_br * temp_br;
                    chain_val += temp_tr * temp_tr * temp_tr; 

                    if temp_r.abs() == 4 {
                        win_val = temp_r;
                    } 
                    else if temp_b.abs() == 4 {
                        win_val = temp_b;
                    } 
                    else if temp_br.abs() == 4 {
                        win_val = temp_br;
                    } 
                    else if temp_tr.abs() == 4 {
                        win_val = temp_tr;
                    }

                }
            }
            
            [win_val.into(), chain_val.into()]
        }
        
        fn value(state: [[i8; 7]; 6], depth: i64, alpha: i64, beta: i64) -> [i64; 2] {
            let MAX_DEPTH = 4; // Less depth = easier
            let val = check_state(state);

            // depth changes the difficulty (less depth = easier)
            if depth >= MAX_DEPTH {

                let win_val = val[0];
                let mut ret_value = val[1] * -1;

                // If it lead to winning, then do it
                if win_val == -4 { // AI win, AI wants to win of course
                    ret_value = 999999; // Less value if it is later on in the game
                } else if win_val == 4 { // AI lose, AI hates losing
                    ret_value = -999999;
                }

                return [ret_value - depth * depth, -1];
            }
            
            let win = val[0];
            // if already won, then return the value right away
            if win == -4 { // AI win, AI wants to win of course
                return [999999 - depth * depth, -1];
            } else if win == 4 { // AI lose, AI hates losing
                return [-999999 - depth * depth, -1];
            }

            if depth % 2 == 0 {
                return min_state(state, depth + 1, alpha, beta);
            }
            return max_state(state, depth + 1, alpha, beta);
        }

        fn choose(choice: Vec<i64>) -> i64 {
            // Use js_sys::Math to find random value (rand does not seem to work)
            return choice[(Math::random() * choice.len() as f64).floor() as usize]
        }

        // Returns [value, choice (column chosen)]
        fn max_state(state: [[i8; 7]; 6], depth: i64, alpha: i64, beta: i64) -> [i64; 2] {
            let mut alpha = alpha;
            let mut v = -100000000007;
            let mut move_col = -1;
            let mut temp_val: [i64; 2];
            let mut move_queue = Vec::<i64>::new();
            for j in 0..7 {
                if let Ok(temp_state) = fill_map(state, j, -1) {
                    temp_val = value(temp_state, depth, alpha, beta);

                    if temp_val[0] > v {
                        v = temp_val[0];
                        move_col = j;
                        move_queue.clear();
                        move_queue.push(j);
                    } else if temp_val[0] == v {
                        move_queue.push(j);
                    }

                    // alpha-beta pruning
                    if v > beta {
                        move_col = choose(move_queue);
                        return [v, move_col];
                    }
                    alpha = max(alpha, v);

                }
            }

            // Randomly choose from move queue
            move_col = choose(move_queue);
            // Return the move to make
            [v, move_col]
        }

        // Returns [value, choice (column chosen)]
        fn min_state(state: [[i8; 7]; 6], depth: i64, alpha: i64, beta: i64) -> [i64; 2] {
            let mut beta = beta;
            let mut v = 100000000007;
            let mut move_col = -1;
            let mut temp_val: [i64; 2];
            let mut move_queue = Vec::<i64>::new();
            for j in 0..7 {
                if let Ok(temp_state) = fill_map(state, j, 1) {
                    temp_val = value(temp_state, depth, alpha, beta);

                    if temp_val[0] < v {
                        v = temp_val[0];
                        move_col = j;
                        move_queue.clear();
                        move_queue.push(j);
                    } else if temp_val[0] == v {
                        move_queue.push(j);
                    }

                    // alpha-beta pruning
                    if v < alpha {
                        move_col = choose(move_queue);
                        return [v, move_col];
                    }
                    beta = min(beta, v);
                }
            }

            // Randomly choose from move queue
            move_col = choose(move_queue);
            // Return the move to make
            [v, move_col]
        }

        // Obtain choice and take action
        let [val, choice] = max_state(curr_state, 0, -100000000007, 100000000007);
        let [val2, choice2] = min_state(curr_state, 0, -100000000007, 100000000007);
        self.action(choice);

        // Print AI's move
        log!("AI -1 choose column: {} (value: {})", choice, val);
        log!("Max state val: {}, column: {}", val, choice);
        log!("Min state val: {}, column: {}", val2, choice2);
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
                log!("In response: {:?}", response);
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
            game_type: "Connect4".into(),
            player1_name: "".into(),
            player2_name: "".into(),
            winner_name: "".into(),
            game_date: 0 // placeholder, when game is saved this can be set
        };
        self.game_started = false;
        self.board.tokens = [[0; 7]; 6];
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
