//! Basic backend

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

use std::collections::HashMap;

use once_cell::sync::OnceCell;
use rocket::{
    fairing::{Fairing, Info, Kind},
    get, post, routes,
    serde::json::Json,
    Build, http::Header, Request, Response, Rocket,
};

use my_sdk::{
    model::{Tournament, TournamentId},
    CreateTournamentRequest, CreateTournamentResponse, GetAllTournamentsResponse,
    GetTournamentResponse, HelloWorldResponse, PerformActionRequest, PerformActionResponse,
};
use tokio::sync::RwLock;
use uuid::Uuid;

//#[cfg(test)]
//mod tests;

static TOURNAMENTS: OnceCell<RwLock<HashMap<TournamentId, Tournament>>> = OnceCell::new();

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/hello/<name>")]
/// A simple "hello world" endpoint
pub fn hello(name: String) -> HelloWorldResponse {
    HelloWorldResponse::new(format!(
        "Hello {name}!! Welcome to the world of fullstack Rust development!!"
    ))
}

#[get("/get/<id>")]
/// Returns a tournament, given an ID
pub async fn get_tournament(id: Uuid) -> GetTournamentResponse {
    GetTournamentResponse::new(
        TOURNAMENTS
            .get()
            .unwrap()
            .read()
            .await
            .get(&id.into())
            .cloned(),
    )
}

#[get("/get/all")]
/// Returns all tournaments
pub async fn get_all_tournaments() -> GetAllTournamentsResponse {
    GetAllTournamentsResponse::new(TOURNAMENTS.get().unwrap().read().await.clone())
}

#[allow(unused_variables)]
#[post("/create", format = "json", data = "<data>")]
/// Creates a new tournament and returns in
pub async fn create_tournament(data: Json<CreateTournamentRequest>) -> CreateTournamentResponse {
    let id = Uuid::new_v4().into();
    let tourn = Tournament::new();
    TOURNAMENTS
        .get()
        .unwrap()
        .write()
        .await
        .insert(id, tourn.clone());
    CreateTournamentResponse::new((id, tourn))
}

#[post("/<id>/action", format = "json", data = "<data>")]
/// Performs an action on a tournament
pub async fn perform_tournament_action(
    id: Uuid,
    data: Json<PerformActionRequest>,
) -> PerformActionResponse {
    PerformActionResponse::new(
        TOURNAMENTS
            .get()
            .unwrap()
            .write()
            .await
            .get_mut(&id.into())
            .map(|tourn| tourn.perform_action(data.0)),
    )
}

/// Creates our Rocket!!
pub fn init() -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .mount("/api/v1/", routes![hello])
        .mount(
            "/api/v1/tournament",
            routes![
                get_tournament,
                get_all_tournaments,
                create_tournament,
                perform_tournament_action
            ],
        )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    TOURNAMENTS
        .set(RwLock::new(HashMap::new()))
        .expect("Could not set TOURNAMENTS");
    let client = init();
    // We don't need to use the rocket directly... yet
    let _rocket = client.launch().await?;
    Ok(())
}
