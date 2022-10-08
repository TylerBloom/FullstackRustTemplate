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

use my_sdk::CreateTournamentRequest;
use yew::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};

use itertools::Itertools;
use once_cell::sync::OnceCell;

use my_sdk::model::{Tournament, TournamentId};

static TOURNAMENTS: OnceCell<RwLock<HashMap<TournamentId, Tournament>>> = OnceCell::new();

enum ViewUpdate {
    CreatedTournament,
}

struct View;

impl Component for View {
    type Message = ViewUpdate;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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
}

struct Create;

impl Component for Create {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        create_tournament();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| ())}>{ "Create Tournament" }</button>
            </div>
        }
    }
}

pub fn main() {
    TOURNAMENTS
        .set(RwLock::new(HashMap::new()))
        .expect("Could not set TOURNAMENTS");
    yew::start_app::<Create>();
    //yew::start_app::<View>();
}

fn get_tournaments() {
    wasm_bindgen_futures::spawn_local(async {
        let resp = reqwest::get("http://localhost:8000/api/v1/tournament/get/all").await.unwrap();
        let map = resp.json().await.unwrap();
        *TOURNAMENTS.get().unwrap().write().unwrap() = map;
        
    });
}

fn create_tournament() {
    wasm_bindgen_futures::spawn_local(async {
        let client = reqwest::Client::new();
        let resp = client.post("http://localhost:8000/api/v1/tournament/create")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&CreateTournamentRequest).unwrap())
            .send()
            .await
            .unwrap();
        let (id, tourn) = resp.json().await.unwrap();
        TOURNAMENTS.get().unwrap().write().unwrap().insert(id, tourn);
    });
}
