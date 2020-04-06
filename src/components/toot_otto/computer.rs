use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Json, Nothing};
use anyhow::Error;
use serde_json::json;
use js_sys::{Date, Math};
use crate::models::game_boards::TootOttoGameBoard;
use std::f64;
use std::cmp::max;

extern crate models;
use models::game::Game;

pub struct TootOttoComputer {
    link: ComponentLink<Self>,
    game: Game,
    selected_disc: char,
    game_started: bool,
    context: Option<CanvasRenderingContext2d>,
    board: TootOttoGameBoard,
    move_num: u8,
    won: bool,
    paused: bool,
    save_task: Option<Result<FetchTask, Error>>
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
    ClickedStart,
    DiscTSelected,
    DiscOSelected,
    ClickedBoard(MouseEvent),
    GetGamesList(Vec<Game>),
    GameSaved,
    SaveError
}

impl Component for TootOttoComputer {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TootOttoComputer {
            link,
            game: Game {
                game_number: 0, // placeholder, when game is saved this can be set
                game_type: "TOOT-OTTO".into(),
                player1_name: "".into(),
                player2_name: "Computer".into(),
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
            save_task: None
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
            Msg::ClickedStart => {
                if self.game.player1_name.is_empty() {
                    //TODO: Show an error message
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
                            // TODO: Perform ai function here 
                            self.ai();
                            // Call self.ai('T') and self.ai('O') then find the max of that??
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
        let canvas = document.get_element_by_id("toot-otto-computer-gameboard").unwrap();
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
                                    id="nameInput1"
                                    type="text"
                                    disabled=self.game_started
                                    value=&self.game.player1_name
                                    oninput=self.link.callback(|e: InputData| Msg::GotPlayer1Input(e.value))
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
                    id="toot-otto-computer-gameboard"
                    height="480"
                    width="640">
                </canvas>
                
            </>
        }
    }
}

impl TootOttoComputer {
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

    fn draw(&self){
        let mut fg_color = "transparent";
        for y in 0..6 {
            for x in 0..7 {
                let mut text = ' ';
                fg_color = "transparent";
                if self.board.tokens[y][x] >= 1 && self.board.disc_map[y][x] == 'T' {
                    fg_color = "#99ffcc";
                    text = 'T';
                } else if self.board.tokens[y][x] >= 1 && self.board.disc_map[y][x] == 'O' {
                    fg_color = "#99ffcc";
                    text = 'O';
                } else if self.board.tokens[y][x] <= -1_i8 && self.board.disc_map[y][x] == 'T' {
                    fg_color = "#ffff99";
                    text = 'T';
                } else if self.board.tokens[y][x] <= -1_i8 && self.board.disc_map[y][x] == 'O' {
                    fg_color = "#ffff99";
                    text = 'O';
                }

                self.draw_circle((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, fg_color.to_string(), "black".to_string(), text.to_string());
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

    fn action(&mut self, column: f64) -> i8 {
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
        context.restore();

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

    fn ai(&self) {
        let curr_state = self.board.disc_map.clone();

        fn fill_map(state: [[char; 7]; 6], column: i64, value: char) -> Result<[[char; 7]; 6], i8> {
            let mut temp_map = state.clone();
            if temp_map[0][column as usize] != '0' || column < 0 || column > 6 {
                return Err(-1);
            }
    
            let mut done = false;
            let mut row = 0;
            for i in 0..5 {
                if (temp_map[i + 1][column as usize] != '0') {
                    done = true;
                    row = i;
                    break;
                }
            }
    
            if !done {
                row = 5;
            }
    
            temp_map[row][column as usize] = value;
            return Ok(temp_map);
        }

        fn check_state(state: [[char; 7]; 6]) -> i8 /*[i8; 2]*/ {
            let mut win_val: i8 = 0;
            let mut chain_val: i8 = 0;

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
                            temp_r[k] = state[i][j + k];
                        }
                        //from (i,j) to bottom
                        if i + k < 6 {
                            temp_b[k] = state[i + k][j];
                        }

                        //from (i,j) to bottom-right
                        if i + k < 6 && j + k < 7 {
                            temp_br[k] = state[i + k][j + k];
                        }

                        //from (i,j) to top-right
                        if (i - k) as i8 >= 0 && j + k < 7 {
                            temp_tr[k] = state[i - k][j + k];
                        }
                    }

                    // Computer wins if spell "OTTO"
                    if temp_r[0] == 'T' && temp_r[1] == 'O' && temp_r[2] == 'O' && temp_r[3] == 'T' {
                        win_val -= 1;
                    }
                    else if temp_r[0] == 'O' && temp_r[1] == 'T' && temp_r[2] == 'T' && temp_r[3] == 'O' {
                        win_val += 1;
                    }
                    else if temp_b[0] == 'T' && temp_b[1] == 'O' && temp_b[2] == 'O' && temp_b[3] == 'T' {
                        win_val -= 1;
                    }
                    else if temp_b[0] == 'O' && temp_b[1] == 'T' && temp_b[2] == 'T' && temp_b[3] == 'O' {
                        win_val += 1;
                    }
                    else if temp_br[0] == 'T' && temp_br[1] == 'O' && temp_br[2] == 'O' && temp_br[3] == 'T' {
                        win_val -= 1;
                    }
                    else if temp_br[0] == 'O' && temp_br[1] == 'T' && temp_br[2] == 'T' && temp_br[3] == 'O' {
                        win_val += 1;
                    }
                    else if temp_tr[0] == 'T' && temp_tr[1] == 'O' && temp_tr[2] == 'O' && temp_tr[3] == 'T' {
                        win_val -= 1;
                    }
                    else if temp_tr[0] == 'O' && temp_tr[1] == 'T' && temp_tr[2] == 'T' && temp_tr[3] == 'O' {
                        win_val += 1;
                    }

                }
            }
            
            win_val
            // [win_val, chain_val]
        }
        
        fn value(state: [[char; 7]; 6], depth: i64, choice: char, alpha: i64, beta: i64) -> i64 {
            let MAX_DEPTH = 4; // Less depth = easier
            let val = check_state(state);
            // log!("Value is called! Depth: {} Value: {} State:\n{:?}", depth, val, state);
            // depth changes the difficulty (less depth = easier)
            if depth >= MAX_DEPTH {
                let mut ret_value: i64 = 0;

                let win_val = val;
                // ret_value = win_val.into(); // ???

                // If it lead to winning, then do it
                if win_val == 1 { // AI win, AI wants to win of course
                    return 999999 - depth * depth; // Less value if it is later on in the game
                } else if win_val == -1 { // AI lose, AI hates losing
                    return -999999 + depth * depth;
                }

                // ret_value -= depth * depth; // ???
                return win_val.into();
            }
            
            let win = val;
            // if already won, then return the value right away
            if win == 1 { // AI win, AI wants to win of course
                // return 999999 - depth * depth;
                return 999999;
            } else if win == -1 { // AI lose, AI hates losing
                // return -999999 - depth * depth;
                return -999999;
            }

            if depth % 2 == 0 {
                return min_state(state, depth + 1, choice, alpha, beta)[0];
            }
            return max_state(state, depth + 1, choice, alpha, beta)[0];
        }

        fn choose(choice: Vec<i64>) -> i64 {
            // Use js_sys::Math to find random value (rand does not seem to work)
            return choice[(Math::random() * choice.len() as f64).floor() as usize]
        }

        // Returns [value, choice (column chosen)]
        fn max_state(state: [[char; 7]; 6], depth: i64, choice: char, alpha: i64, beta: i64) -> [i64; 3] {
            let mut alpha = alpha;
            let mut v = -100000000007;
            let mut move_num = -1;
            let mut temp_val: i64;
            let mut move_queue = Vec::<i64>::new();
            let mut selected_disc = 0; // 0 for 'T'
            for j in 0..7 {
                // TODO: do both states? T or O -> choice
                if let Ok(temp_state) = fill_map(state, j, choice) {
                    temp_val = value(temp_state, depth, choice, alpha, beta);

                    if temp_val > v {
                        v = temp_val;
                        move_num = j;
                        selected_disc = 0;
                        move_queue.clear();
                        move_queue.push(j);
                    } else if temp_val == v {
                        move_queue.push(j);
                    }

                    // alpha-beta pruning
                    if v > beta {
                        move_num = choose(move_queue);
                        return [v, move_num, 0];
                    }
                    alpha = max(alpha, v);

                }

                // if let Ok(temp_state) = fill_map(state, j, choice) {
                //     temp_val = value(temp_state, depth, 'O', alpha, beta);

                //     if temp_val > v {
                //         v = temp_val;
                //         move_num = j;
                //         selected_disc = 1;
                //         move_queue.clear();
                //         move_queue.push(j);
                //     } else if temp_val == v {
                //         move_queue.push(j);
                //     }

                //     // alpha-beta pruning
                //     if v > beta {
                //         move_num = choose(move_queue);
                //         return [v, move_num, 1];
                //     }
                //     alpha = max(alpha, v);

                // }
            }
            // Randomly choose from move queue
            move_num = choose(move_queue);
            // Return the move to make
            [v, move_num, selected_disc]
        }

        // Returns [value, choice (column chosen)]
        fn min_state(state: [[char; 7]; 6], depth: i64, choice: char, alpha: i64, beta: i64) -> [i64; 3] {
            let mut alpha = alpha;
            let mut v = 100000000007;
            let mut move_num = -1;
            let mut temp_val: i64;
            let mut move_queue = Vec::<i64>::new();
            let mut selected_disc = 0; // 0 for 'T', 1 for 'O'
            for j in 0..7 {
                // TODO: fix choice value...? -> should be human player's choice
                if let Ok(temp_state) = fill_map(state, j, choice) {
                    temp_val = value(temp_state, depth, choice, alpha, beta);

                    if temp_val < v {
                        v = temp_val;
                        move_num = j;
                        selected_disc = 0;
                        move_queue.clear();
                        move_queue.push(j);
                    } else if temp_val == v {
                        move_queue.push(j);
                    }

                    // alpha-beta pruning
                    if v < alpha {
                        move_num = choose(move_queue);
                        return [v, move_num, 0];
                    }
                    alpha = max(alpha, v);

                }

                // if let Ok(temp_state) = fill_map(state, j, choice) {
                //     temp_val = value(temp_state, depth, 'O', alpha, beta);

                //     if temp_val < v {
                //         v = temp_val;
                //         move_num = j;
                //         selected_disc = 1;
                //         move_queue.clear();
                //         move_queue.push(j);
                //     } else if temp_val == v {
                //         move_queue.push(j);
                //     }

                //     // alpha-beta pruning
                //     if v < alpha {
                //         move_num = choose(move_queue);
                //         return [v, move_num, 1];
                //     }
                //     alpha = max(alpha, v);

                // }
            }
            // Randomly choose from move queue
            move_num = choose(move_queue);
            // Return the move to make
            [v, move_num, selected_disc]
        }

        // TODO: find max value between both choices and take -> if lose, then??
        let disc_choices = ['T', 'O'];

        let choice_val_T = max_state(curr_state, 0, 'T', -100000000007, 100000000007);
        let val_T = choice_val_T[0];
        let selected_choice_T = choice_val_T[1];
        log!("AI Disc type: T chosen column: {} (value: {}) disc: {}", selected_choice_T, val_T, disc_choices[choice_val_T[2] as usize]);

        let choice_val_O = max_state(curr_state, 0, 'O', -100000000007, 100000000007);
        let val_O = choice_val_O[0];
        let selected_choice_O = choice_val_O[1];
        log!("AI Disc type: O chosen column: {} (value: {}), disc: {}", selected_choice_O, val_O, disc_choices[choice_val_O[2] as usize]);

        // Make move based on information

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
            player2_name: "Computer".into(),
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
