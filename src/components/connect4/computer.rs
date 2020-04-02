use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::f64;

// TODO: import this instead of copying/pasting, waiting for henry to merge models folder
pub struct Game {
    pub game_number: i32,
    pub game_type: String,
    pub player1_name: String,
    pub player2_name: String,
    pub winner_name: String,
    pub game_date: i64
}

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    game: Game,
    game_started: bool,
    context: Option<web_sys::CanvasRenderingContext2d>
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {
    GotInput(String),
    ClickedStart
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
            context: None,
            game_started: false
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
            }
        }
        true
    }
    

    fn mounted(&mut self) -> ShouldRender {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("gameboard").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        self.context = Some(canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
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

                <canvas id="gameboard" height="480" width="640"></canvas>
                
            </>
        }
    }
}

impl Connect4Computer {
    fn draw_board(&self) {
        let context = self.context.as_ref().unwrap();

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
}