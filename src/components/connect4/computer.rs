use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::f64;

extern crate models;
use models::game::Game;

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    game: Game,
    game_started: bool,
    context: Option<CanvasRenderingContext2d>,
    board: GameBoard,
    moveNum: u8,
    won: bool,
    paused: bool,
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
            board: GameBoard {
                rows: 6,
                columns: 7,
                tokens: [[0; 7]; 6],
            },
            moveNum: 0,
            won: false,
            paused: false,
        }
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.game.player1_name = new_value;
            }
            Msg::ClickedStart => {
                if self.game.player1_name.is_empty() {
                    //TODO: Show an error message
                } else {
                    self.game_started = true;
                    self.draw_board();
                }
            },
            Msg::ClickedBoard(event) => {
                if !self.game_started {
                    return false;
                }
                let rect = self.canvas().get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();
                log!("x: {} y: {}", x,y);

                for i in 0..7 {
                    if self.on_region([x,y], (75 * i + 100) as f64, 25.0){
                        log!("Region {} clicked", i);
                        let valid = self.action(i as f64);
                        if valid == 1 {
                            //Reject Click
                        }
                    }
                    
                }

                // for (j = 0; j < 7; j++) {
                //     if (this.onregion([x, y], 75 * j + 100, 25)) {
                //         // console.log("clicked region " + j);
                //         this.paused = false;
                        
                //         valid = this.action(j, function () {
                //             that.ai(-1);
                //         }); 
                //         if (valid === 1) { // give user retry if action is invalid
                //             this.rejectClick = true;
                //         }
                //         break; //because there will be no 2 points that are clicked at a time
                //     }
                // }
            }
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

    fn draw(&self){
        let mut fg_color = "transparent";
        for y in 0..6 {
            for x in 0..7 {
                fg_color = "transparent";
                if self.board.tokens[y][x] >= 1 {
                    fg_color = "#ff4136";
                } else if self.board.tokens[y][x] <= -1_i8 {
                    fg_color = "#ffff00";
                }
                self.drawCircle((75 * x + 100) as f64, (75 * y + 50) as f64, 25.0, fg_color.to_string(), "black".to_string());
            }
        }
    }

    fn drawCircle(&self, x: f64,y: f64, r: f64, fill: String, stroke: String){
        log!("drawing circle");
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

    fn on_region(&self, coord: [f64;2], x: f64, radius: f64) -> bool{
        if (coord[0] - x as f64)*(coord[0] - x as f64) <=  radius * radius {
            return true;
        }
        return false;
    }

    // fn animate(&self, column: f64, moveM: u8, row: f64, cur_pos: f64){

    // }


    fn action(&mut self, column: f64) -> i8{
        if self.paused || self.won {
            return 0;
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
        log!("Adding token to row {}", row);
        self.board.tokens[row as usize][column as usize] = self.playerToken();
        self.draw();
        self.check();
        // self.animate(column, self.moveNum, row as f64, 0.0);
        
        // Set pause to true to do AI move
        // self.paused = true;
        return 1;
    }

    fn check (&self){
        let mut temp_r = 0;
        let mut temp_b = 0;
        let mut temp_br = 0;
        let mut temp_tr = 0;

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
        if self.moveNum == 42 && !self.won {
            self.win(0);
        }
    }

    fn win (&self, player: i8){
        if player > 0 {
            log!("Player wins");
        }
        else if player < 0{
            log!("Computer wins");
        }
        else{
            log!("Draw");
        }

    }


    fn playerToken (&self) -> i8{
        if self.moveNum %2 == 0 {
            return 1;
        }else{
            return -1;
        }
    }

    fn context(&self) -> &CanvasRenderingContext2d {
        self.context.as_ref().unwrap()
    }

    fn canvas(&self) -> HtmlCanvasElement {
        self.context.as_ref().unwrap().canvas().unwrap()
    }
}
