extern crate curl;
extern crate serde;
extern crate chrono;
extern crate serde_json;
extern crate plotlib;
extern crate colored;
#[macro_use] extern crate serde_derive;

use curl::easy::{Easy2, Handler, WriteError};
use std::collections::HashMap;

use plotlib::style::Line;

use std::fs::File;
use std::io::Write;

use colored::*;

fn main() {
    let mut curl = Easy2::new(Collector(Vec::new()));
    curl.get(true).unwrap();
    curl.url("https://statsapi.web.nhl.com/api/v1/schedule").unwrap();
    curl.perform().unwrap();

    let web = curl.get_ref();
    let json = String::from_utf8(web.0.as_slice().to_vec()).unwrap();

    let data: Api = serde_json::from_str(&json).unwrap();

    println!();
    println!("🏒🥅🏒🥅🏒🥅🏒🥅");
    println!();

    for date in data.dates {
        for game in date.games {
            if game.status.detailedState == "Final" {
                if game.teams.away.score > game.teams.home.score {
                    println!("{} @ {}",
                    game.teams.away.team.name.green().bold(),
                    game.teams.home.team.name.bold())
                } else if game.teams.away.score < game.teams.home.score {
                    println!("{} @ {}",
                    game.teams.away.team.name.bold(),
                    game.teams.home.team.name.green().bold())
                }
            } else {
                println!("{} @ {}", game.teams.away.team.name.bold(), game.teams.home.team.name.bold());
            }
            println!("{} - {}",
            game.teams.away.score,
            game.teams.home.score);
            if game.status.detailedState == "Final" {
                println!("{}",
                game.status.detailedState.green());
            } else {
                println!("{}",
                game.status.detailedState.magenta());
            }
        }
    }

    println!();
    println!("🏒🥅🏒🥅🏒🥅🏒🥅");
    println!();
}


struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

// Define the api datastructure
#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Api {
    copyright: String,
    totalItems: usize,
    totalEvents: usize,
    totalGames: usize,
    totalMatches: usize,
    wait: usize,
    dates: Vec<ApiDate>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiDate {
    date: String,
    totalItems: usize,
    totalEvents: usize,
    totalGames: usize,
    totalMatches: usize,
    games: Vec<ApiGame>,
    events: Vec<()>,
    matches: Vec<()>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiGame {
    gamePk: usize,
    link: String,
    gameType: String,
    season: String,
    gameDate: String,
    status: ApiStatus,
    teams: ApiTeams,
    venue: ApiVenue,
    content: ApiContent,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiStatus {
    abstractGameState: String,
    codedGameState: String,
    detailedState: String,
    statusCode: String,
    startTimeTBD: bool,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeams {
    away: ApiTeamResult,
    home: ApiTeamResult,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeamResult {
    score: usize,
    team: ApiTeam,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiTeam {
    id: usize,
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiVenue {
    name: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ApiContent {
    link: String,
}