use yew::prelude::*;
use crate::models::game::Game;

pub struct GameHistory {
    games: Vec<Game>,
    link: ComponentLink<Self>
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub enum Msg {}

impl Component for GameHistory {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // Get games from API
        GameHistory {
            games: Vec::<Game>::new(),
            link: link
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
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
                        // <tr ng-repeat="game in games">
                        //     <td>{{ $index + 1 }}</td>
                        //     <td>{{game.gameType}}</td>
                        //     <td>{{game.Player1Name}}</td>
                        //     <td>{{game.Player2Name}}</td>
                        //     <td>{{game.WinnerName}}</td>
                        //     <td>{{game.GameDate | date:"h:mma 'on' MMM d, y"}}</td>
                        // </tr>
                    </table>
                </div>
            </div>
        }
    }
}