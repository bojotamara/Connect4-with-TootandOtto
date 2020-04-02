use yew::prelude::*;
use yew::services::fetch::{Request, Response, FetchService, FetchTask};
use yew::format::{Nothing, Json};
// use yew::services::console::ConsoleService;
use chrono::NaiveDateTime;
use anyhow::Error;
use crate::models::game::Game;

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
            <div class="w3-container" id="services" style="margin-top:75px">
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
                        { self.populate_table() }
                    </table>
                </div>
            </div>
        }
    }
}

impl GameHistory {
    fn render_game_row(&self, index: usize) -> Html {
        match self.games.get(index) {
            Some(game) => html!{
                <tr>
                    <td>{{ index + 1 }}</td>
                    <td>{{ game.game_type.clone() }}</td>
                    <td>{{ game.player1_name.clone() }}</td>
                    <td>{{ game.player2_name.clone() }}</td>
                    <td>{{ game.winner_name.clone() }}</td>
                    <td>{{ Self::convert_timestamp(game.game_date) }}</td> // TODO: parse date to string
                </tr>
            },
            None => html!{<></>},
        }
    }

    fn render_games(&self, index: usize) -> Html {
        if index == self.games.len() - 1 {
            self.render_game_row(index)
        } else {
            html! {
                <>
                    {{ self.render_game_row(index) }}
                    {{ self.render_games(index + 1) }}
                </>
            }
        }
    }

    fn populate_table(&self) -> Html {
        if self.games.len() != 0 {
            return self.render_games(0);
        }
        return html!{<></>}
    }

    fn convert_timestamp(timestamp: i64) -> String {
        // timestamp in milliseconds
        let dt = NaiveDateTime::from_timestamp(timestamp / 1000, 0);
        format!("{}", dt.format("%l:%M%p on %b %e, %Y"))
    }
}