//! This library wraps our business logic library, `my_lib`, and adds in types for client-server
//! communication. The goal of this library is to provide a single point of entry for programs
//! needing both request/response types and business logic.

#![deny(
    unused,
    irrefutable_let_patterns,
    missing_docs,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub
)]
#![warn(rust_2018_idioms)]

/// From the users' side (namely the front and back ends), this module contains everything that the
/// business logic library contains and renames it to `model`.
pub use my_lib as model;

/// Contains the errors used by this crate
pub mod error;
/// Contains the types needed for the `api/v1/hello` endpoint
pub mod hello;
/// Contains the wrapper response type
pub mod response;