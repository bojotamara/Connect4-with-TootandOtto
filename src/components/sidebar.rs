use yew::prelude::*;

pub struct Sidebar {
    link: ComponentLink<Self>
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Sidebar {link}
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="w3-sidenav w3-red w3-bar-block w3-large" style="width:25%;font-weight:bold;position:fixed;height:100%;overflow:auto;" id="sidenav">
                <div class="w3-container">
                    <h3 class="w3-padding-64"><b>{"Play Connect4 / TOOT-OTTO"}</b></h3>
                </div>
                <a href="#/HowToConnect4" class="w3-bar-item w3-button w3-hover-white">{"How to Play Connect4"}</a>
                <a href="#/Connect4Computer" class="w3-bar-item w3-button w3-hover-white">{"Play Connect4 With Computer"}</a> 
                <a href="#/Connect4Human" class="w3-bar-item w3-button w3-hover-white">{"Play Connect4 with Another Human"}</a>
                <h5 class="w3-bar-item"></h5>
                <a href="#/HowToToot" class="w3-bar-item w3-button w3-hover-white">{"How to Play TOOT-OTTO"}</a>
                <a href="#/TootOttoComputer" class="w3-bar-item w3-button w3-hover-white">{"Play Toot-Otto With Computer"}</a>
                <a href="#/TootOttoHuman" class="w3-bar-item w3-button w3-hover-white">{"Play Toot-Otto With Another Human"}</a>
                <h5 class="w3-bar-item"></h5>
                <a href="#/ScoreBoard" class="w3-bar-item w3-button w3-hover-white">{"View Game History"}</a>
                <a href="#/Scores" class="w3-bar-item w3-button w3-hover-white">{"Score Board"}</a>
            </nav>
        }
    }
}