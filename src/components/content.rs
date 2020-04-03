use yew::prelude::*;
use super::game_history::GameHistory;
use super::score_board::ScoreBoard;
use super::connect4::computer::{Connect4Computer, Connect4Human};

#[derive(Clone, PartialEq)]
pub enum Tab {
    Home,
    HowToConnect4,
    Connect4Computer,
    Connect4Human,
    HowToToot,
    TootOttoComputer,
    TootOttoHuman,
    ScoreBoard,
    Scores,
}

pub struct Content {
    props: Props,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub tab: Tab,
}

fn get_tab_html(tab: &Tab) -> Html {
    match tab {
        Tab::Home => html!{ {"You are home!"} },
        Tab::HowToConnect4 => html!{ {"2"} },
        Tab::Connect4Computer => html!{ <Connect4Computer/> },
        Tab::Connect4Human => html!{ {"4"} },
        Tab::HowToToot => html!{ {"5"} },
        Tab::TootOttoComputer => html!{ {"6"} },
        Tab::TootOttoHuman => html!{ {"7"} },
        Tab::ScoreBoard => html!{<GameHistory />},
        Tab::Scores => html!{<ScoreBoard />}
    }
}

// Message represents a variety of messages that can be processed by the component 
// to trigger some side effect. For example, you may have a Click message which triggers
// an API request or toggles the appearance of a UI component.
pub enum Msg {}

impl Component for Content {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Content {props, link}
    }

    // Update life cycle method is called for each asynchronous message
    // Messages can be triggered by HTML elements listeners or be sent by child components,
    // Agents, Services, or Futures.
    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true // This will always re-render when new props are provided.
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-main" style="margin-left:28%;height:100%;overflow:auto;">
                {get_tab_html(&self.props.tab)}
            </div>
        }
    }
}