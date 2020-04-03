use yew::prelude::*;
use chrono::NaiveDateTime;
use crate::models::game::Game;
use std::collections::HashMap;

/**
 * Game History Table
 */

fn render_gh_row(games: &Vec<Game>, index: usize) -> Html {
    match games.get(index) {
        Some(game) => html!{
            <tr>
                <td>{{ index + 1 }}</td>
                <td>{{ game.game_type.clone() }}</td>
                <td>{{ game.player1_name.clone() }}</td>
                <td>{{ game.player2_name.clone() }}</td>
                <td>{{ game.winner_name.clone() }}</td>
                <td>{{ convert_timestamp(game.game_date) }}</td>
            </tr>
        },
        None => html!{<></>},
    }
}

fn render_all_gh_rows(games: &Vec<Game>, index: usize) -> Html {
    if index == games.len() - 1 {
        render_gh_row(games, index)
    } else {
        html! {
            <>
                {{ render_gh_row(games, index) }}
                {{ render_all_gh_rows(games, index + 1) }}
            </>
        }
    }
}

pub fn render_gh_table(games: &Vec<Game>) -> Html {
    if games.len() != 0 {
        return render_all_gh_rows(games, 0);
    }
    return html!{<></>}
}

/**
 * Score Board Tables
 */

// Computer wins

pub fn render_computer_wins_table(games: &Vec<Game>) -> Html {
    let total_games_played = games.len();
    let games_against_computer = games.into_iter().filter(|game| game.player2_name == "Computer").count();
    let computer_wins = games.into_iter().filter(|game| game.winner_name == "Computer").count();
    html!(
        <tr>
            <td>{{ total_games_played }}</td>
            <td>{{ games_against_computer }}</td>
            <td>{{ computer_wins }}</td>
        </tr>
    )
}

// Computer wins details

fn render_cw_row(games: &Vec<Game>, index: usize) -> Html {
    match games.get(index) {
        Some(game) => html!{
            <tr>
                <td>{{ index + 1 }}</td>
                <td>{{ game.game_type.clone() }}</td>
                <td>{{ game.winner_name.clone() }}</td>
                <td>{{ game.player1_name.clone() }}</td>
                <td>{{ convert_timestamp(game.game_date) }}</td>
            </tr>
        },
        None => html!{<></>},
    }
}

fn render_all_cw_rows(games: &Vec<Game>, index: usize) -> Html {
    if index == games.len() - 1 {
        render_cw_row(games, index)
    } else {
        html!{
            <>
                {{ render_cw_row(games, index) }}
                {{ render_all_cw_rows(games, index + 1) }}
            </>
        }
    }
}

pub fn render_cw_table(games: &Vec<Game>) -> Html {
    let mut computer_wins = Vec::<Game>::new();
    games.iter()
        .filter(|game| game.winner_name == "Computer")
        .for_each(|game| computer_wins.push(game.clone()));
    if computer_wins.len() != 0 {
        return render_all_cw_rows(&computer_wins, 0);
    }
    return html!{<></>}
}

// All games won details

fn render_gw_row(winners: &Vec<(String, usize)>, index: usize) -> Html {
    match winners.get(index) {
        Some((player, wins)) => html!{
            <tr>
                <td>{{ index + 1 }}</td>
                <td>{player}</td>
                <td>{wins}</td>
            </tr>
        },
        None => html!{<></>},
    }
}

fn render_all_gw_rows(winners: &Vec<(String, usize)>, index: usize) -> Html {
    if index == winners.len() - 1 {
        render_gw_row(winners, index)
    } else {
        html!{
            <>
                {{ render_gw_row(winners, index) }}
                {{ render_all_gw_rows(winners, index + 1) }}
            </>
        }
    }
}

pub fn render_gw_table(games: &Vec<Game>) -> Html {
    // Generate a vector with names of all the winners
    let winners_list: Vec<String> = games.into_iter().map(|game| game.winner_name.clone()).collect();
    // Create hashmap for win count
    let mut player_wins: HashMap<String, usize> = HashMap::<String, usize>::new();
    // Increment counter for each time a player's name shows up
    winners_list.iter().for_each(|player| {
        let counter = player_wins.entry(player.clone()).or_insert(0);
        *counter += 1;
    });
    // Generate vector from hashmap
    let mut winners: Vec<(String, usize)> = player_wins.into_iter().map(|player| player).collect();
    // Sort by most to least wins
    winners.sort_by(|a, b| b.1.cmp(&a.1));
    // Generate the table rows
    if winners.len() != 0 {
        return render_all_gw_rows(&winners, 0);
    }
    return html!{<></>}
}

// TODO: Fix time conversion to Local timezone

fn convert_timestamp(timestamp: i64) -> String {
    // timestamp in milliseconds
    let dt = NaiveDateTime::from_timestamp(timestamp / 1000, 0);
    format!("{}", dt.format("%l:%M%p on %b %e, %Y"))
}