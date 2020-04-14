use yew::prelude::*;
use super::content::Tab;

pub struct Sidebar {
    link: ComponentLink<Self>,
    onsignal: Callback<Tab>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onsignal: Callback<Tab>,
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {
    TabClicked(Tab)
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Sidebar {
            link: link,
            onsignal: props.onsignal
        }
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TabClicked(tab) => {
                self.onsignal.emit(tab)
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="w3-sidenav w3-red w3-bar-block w3-large" style="width:25%;font-weight:bold;position:fixed;height:100%;overflow:auto;" id="sidenav">
                <div class="w3-container">
                    <h3 style="padding-top:48px;padding-bottom:30px;"><b>{"Play Connect4 / TOOT-OTTO"}</b></h3>
                </div>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::Home))
                    class="w3-bar-item w3-button w3-hover-white active">
                    {"Home"}
                </button>
                <h5 class="w3-bar-item"></h5>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::HowToConnect4))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"How to Play Connect4"}
                </button>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::Connect4Computer))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"Play Connect4 With Computer"}
                </button> 
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::Connect4Human))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"Play Connect4 with Another Human"}
                </button>
                <h5 class="w3-bar-item"></h5>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::HowToToot))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"How to Play TOOT-OTTO"}
                </button>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::TootOttoComputer))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"Play Toot-Otto With Computer"}
                </button>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::TootOttoHuman))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"Play Toot-Otto With Another Human"}
                </button>
                <h5 class="w3-bar-item"></h5>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::ScoreBoard))
                    class="w3-bar-item w3-button w3-hover-white">
                    {"View Game History"}
                </button>
                <button
                    onclick=self.link.callback(|_| Msg::TabClicked(Tab::Scores))
                    class="w3-bar-item w3-button w3-hover-white">{"Score Board"}
                </button>
            </nav>
        }
    }
}