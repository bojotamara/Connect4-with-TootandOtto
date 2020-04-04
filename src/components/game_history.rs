use yew::prelude::*;
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Nothing, Json};
// use yew::services::console::ConsoleService;
use anyhow::Error;
use crate::models::game::Game;
use super::utils::table_builder;

pub struct GameHistory {
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

impl Component for GameHistory {
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

        GameHistory {
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
            <div class="w3-container" id="services" style="margin-top:75px;margin-bottom:75px;">
                <h5 class="w3-xxxlarge w3-text-red"><b>{"Game History"}</b></h5>
                <hr style="width:50px;border:5px solid red" class="w3-round" />
                
                <div id="game-stream">
                    <table>
                        <tr>
                            <th>{"Game-ID"}</th>
                            <th>{"Game Type"}</th>
                            <th>{"Player1"}</th>
                            <th>{"Player2"}</th>
                            <th>{"Winner"}</th>
                            <th>{"When Played"}</th>
                        </tr>
                        { table_builder::render_gh_table(&self.games) }
                    </table>
                </div>
            </div>
        }
    }
}