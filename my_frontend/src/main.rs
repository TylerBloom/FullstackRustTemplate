//! Basic frontent

/*
#![deny(
    unused,
    irrefutable_let_patterns,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]
#![warn(rust_2018_idioms)]
*/

use std::{collections::HashMap, sync::RwLock};

use yew::prelude::*;

use reqwasm::http::Request;
use itertools::Itertools;
use once_cell::sync::OnceCell;
use web_sys::{console};
use tokio::runtime;

use my_sdk::model::{Tournament, TournamentId};

static TOURNAMENTS: OnceCell<RwLock<HashMap<TournamentId, Tournament>>> = OnceCell::new();

#[function_component(App)]
fn app() -> Html {
    let lock = TOURNAMENTS.get().unwrap().read().unwrap();
    lock.iter().map(|(id, tourn)| 
        html! {
        <>
            <h1>{ "Tournament Manager" }</h1>
            <div>
                <h2>{ format!("Id: {}", id) }</h2>
                <hr/>
                <h2>{ "Players:" }</h2>
                { tourn.get_players().map(|(_, plyr)| html! { <p>{ format!("Player: {}", plyr.name) }</p> }).collect::<Html>() }
                <hr/>
                <h2>{ "Games:" }</h2>
                { tourn.get_games().map(|(_, game)| 
                    html! { 
                        <div>
                            <p>{ "Game" }</p> 
                            <p>{ format!("Players: {}", game.players.iter().map(|p| tourn.get_player(p).map(|p| &p.name).unwrap()).join(", ")) }</p> 
                            <p>{ format!("Winner: {}", game.winner.map(|w| tourn.get_player(&w).map(|p| p.name.as_str())).flatten().unwrap_or("None")) }</p> 
                        </div>
                    }).collect::<Html>()
                }
            </div>
        </>
        }
        ).collect()
}

pub fn main() {
    console::log_1(&"Starting up!!".into());
    let rt  = runtime::Builder::new_current_thread().build().expect("Could not create tokio runtime");
    let data = rt.block_on(async {
        Request::get("127.0.0.1:8000/api/v1/tournament/get/all")
            .send()
            .await
            .expect("Could not contact backend")
    });
    let _json = rt.block_on(async move {
        data
            .json::<HashMap<TournamentId, Tournament>>()
            .await
            .expect("Could not serialize backend response")
    });
    console::log_1(&"Successful request".into());
    /*
    let map: HashMap<_, _> = response.unwrap().json().await.expect("Could not decode json");
    console::log_1(&"JSON Successfully decoded".into());
    TOURNAMENTS.set(RwLock::new(map)).expect("Could not set TOURNAMENTS");
    yew::start_app::<App>();
    */
}
