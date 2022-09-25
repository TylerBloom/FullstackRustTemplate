//! Basic backend

#![deny(
    unused,
    irrefutable_let_patterns,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]
#![warn(rust_2018_idioms)]

use my_sdk::hello::HelloWorldResponse;
use rocket::{get, routes, Build, Rocket};

//#[cfg(test)]
//mod tests;

#[get("/hello/<name>")]
/// A simple "hello world" endpoint
pub fn hello_world(name: String) -> HelloWorldResponse {
    HelloWorldResponse::new(format!("Hello {name}!! Welcome to the world of fullstack Rust development!!"))
}

/// Creates our Rocket!!
pub fn init() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api/v1/",
            routes![hello_world],
        )
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let client = init();
    // We don't need to use the rocket directly... yet
    let _rocket = client.launch().await?;
    Ok(())
}
