use yew::prelude::*;
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Nothing, Json};
use anyhow::Error;
use crate::models::game::Game;
use super::utils::table_builder;

pub struct ScoreBoard {
    games: Vec<Game>,
    link: ComponentLink<Self>,
    get_games_task: Result<FetchTask, Error>, // Important to keep in scope!!
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub enum Msg {
    FetchResourceComplete(Vec<Game>),
    FetchResourceFailed
}

impl Component for ScoreBoard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        // Create GET request for list of games
        let get_request = Request::get("http://localhost:8000/list-games")
            .body(Nothing)
            .unwrap();

        // Create task for FetchService
        let task = FetchService::new().fetch(
            get_request,
            link.callback(|response: Response<Json<Result<Vec<Game>, Error>>>| {
                if let (meta, Json(Ok(body))) = response.into_parts() {
                    if meta.status.is_success() {
                        return Msg::FetchResourceComplete(body);
                    }
                }
                Msg::FetchResourceFailed
            }),
        );

        ScoreBoard {
            games: Vec::<Game>::new(),
            link: link,
            get_games_task: task // Note: Reference to task needs to be stored for the duration of the request (https://github.com/yewstack/yew/issues/388)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchResourceComplete(games) => {
                for game in games {
                    self.games.push(game);
                }
                return true;
            },
            Msg::FetchResourceFailed => return false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="w3-container" id="services" style="margin-top:75px">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Score Board"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                <div><h4>{"Games Won by Computer"}</h4></div>
                <table>
                        <tr>
                            <th>{"Total Games Played"}</th>
                            <th>{"Games Against Computer"}</th>
                            <th>{"Games Computer Won"}</th>
                        </tr>
                        { table_builder::render_computer_wins_table(&self.games) }
                </table>

                <br></br>

                <div><h4>{"Details of Games Won by Computer"}</h4></div>
                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Winner"}</th>
                            <th>{"Played Against"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        { table_builder::render_cw_table(&self.games) }
                    </table>
                </div>

                <br></br>

                <div><h4>{"Details of Games Won by All Players"}</h4></div>
                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Sl. No."}</th>
                            <th>{"Winner or Draw"}</th>
                            <th>{"No. of Wins"}</th>
                        </tr>
                        { table_builder::render_gw_table(&self.games) }
                    </table>
                </div>
            </div>
        }
    }
}