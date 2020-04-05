use yew::prelude::*;
use super::game_history::GameHistory;
use super::score_board::ScoreBoard;
use super::connect4::{computer::Connect4Computer, human::Connect4Human};
use super::toot_otto::{human::TootOttoHuman};

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

fn homepage_html() -> Html {
    html!{
        <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"Welcome"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round" />
            <p>{"This application contains the following two board games, both in human Vs. human and human Vs. Computer versions."}</p>
            <ul>
                <li>{"Connect 4"}</li>
                <li>{"TOOT-OTTO"}</li>
            </ul>
            <p>{"Select the game of your choice from the side bar, and start playing. Enjoy!"}</p>
        </div>
    }
}

fn how_to_connect4_html() -> Html {
    html!{
        <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play Connect 4"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round" />
            <p>{"Connect Four is a two-player connection game in which the players take turns dropping colored discs from the top into a seven-column, six-row vertically suspended grid. The objective of the game is to be the first to form a horizontal, vertical, or diagonal line of four of one's own discs."}</p>
            <br></br>
            <div><h5>{"To play Connect 4 follow the following steps:"}</h5></div>
            <ul>
                <li>{"A new game describes discs of which color belongs to which player"}</li>
                <li>{"Click on the desired column on the game board to place your disc"}</li>
                <li>{"Try to connect 4 of your colored discs either horizontally or vertically or diagonally"}</li>
            </ul>
            <br></br>
            <p>{"For More information on Connect 4 click "}<a href="https://en.wikipedia.org/wiki/Connect_Four">{"here"}</a></p>
        </div>
    }
}

fn how_to_toot_html() -> Html {
    html!{
        <div class="w3-container" id="services" style="margin-top:75px">
            <h5 class="w3-xxxlarge w3-text-red"><b>{"How to Play TOOT-OTTO"}</b></h5>
            <hr style="width:50px;border:5px solid red" class="w3-round" />
            <p>{"TOOT-OTTO is a fun strategy game for older players who like tic-tac-toe and checkers. One player is TOOT and the other player is OTTO. Both players can place both T's and O's, based on their choice. The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}</p>
            <br></br>
            <div><h5>{"To play TOOT-OTTO follow the following steps:"}</h5></div>
            <ul>
                <li>{"A new game describes which player is TOOT and which is OTTO"}</li>
                <li>{"Select the disc type T or O that you want to place"}</li>
                <li>{"Click on the desired column on the game board to place your disc"}</li>
                <li>{"Try to spell TOOT or OTTO based on your winning combination, either horizontally or vertically or diagonally"}</li>
            </ul>
            <br></br>
            <p>{"For More information on TOOT-OTTO click "}<a href="https://boardgamegeek.com/boardgame/19530/toot-and-otto">{"here"}</a></p>
        </div>
    }
}

fn get_tab_html(tab: &Tab) -> Html {
    match tab {
        Tab::Home => homepage_html(),
        Tab::HowToConnect4 => how_to_connect4_html(),
        Tab::Connect4Computer => html!{ <Connect4Computer /> },
        Tab::Connect4Human => html!{ <Connect4Human /> },
        Tab::HowToToot => how_to_toot_html(),
        Tab::TootOttoComputer => html!{ {"6"} },
        Tab::TootOttoHuman => html!{ <TootOttoHuman /> },
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