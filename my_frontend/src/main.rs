//! Basic frontent

#![deny(
    unused,
    irrefutable_let_patterns,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]
#![warn(rust_2018_idioms)]

use std::{collections::HashMap, sync::RwLock};

use yew::prelude::*;

use itertools::Itertools;
use once_cell::sync::OnceCell;
use reqwasm::http::Request;

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

#[tokio::main(flavor = "current_thread")]
async fn main() {
    TOURNAMENTS
        .set(RwLock::new(
            Request::get("127.0.0.1:8000/api/v1/tournament/all")
                .send()
                .await
                .expect("Could not contact backend")
                .json()
                .await
                .expect("Could not serialize backend response"),
        ))
        .expect("Could not set TOURNAMENTS");
    yew::start_app::<App>();
}
