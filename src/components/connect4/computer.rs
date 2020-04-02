use yew::prelude::*;

pub struct Connect4Computer {
    link: ComponentLink<Self>,
    player_name: String,
    game_started: bool
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
        Connect4Computer {link, player_name: "".into(), game_started: false}
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.player_name = new_value;
            }
            Msg::ClickedStart => {
                if self.player_name.is_empty() {
                    //show an error message
                } else {
                    self.game_started = true;
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
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
                                value=&self.player_name
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
        }
    }
}