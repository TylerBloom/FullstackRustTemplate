#[cfg(feature = "rocket")]
use std::io::Cursor;

use rocket::http::Status;
#[cfg(feature = "rocket")]
use rocket::{
    response::{Responder, Result as RResult},
    Response,
};

use serde::Deserialize;
#[cfg(feature = "rocket")]
use serde::Serialize;

/// A shorthand used by SquireCore
pub const SERIALIZER_ERROR: u16 = 500;

#[derive(Debug, Deserialize)]
/// Rocket requires the `Responder` trait to return a type as a response.
/// This is the basic wrapper struct that implements `Responder` so long as its wrapped data
/// implements serde's `Serialize` and `Deserialize`.
pub struct BasicResponse<T>(pub T);

impl<T> BasicResponse<T> {
    /// Creates a new `BasicResponse` object
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

// Note, the `Responder` trait is only needed by the backend.
// We ignore this for non-Rocket applications
#[cfg(feature = "rocket")]
impl<'r, T> Responder<'r, 'r> for BasicResponse<T>
where
    T: Serialize + Deserialize<'r>,
{
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> RResult<'r> {
        serde_json::to_string(&self.0)
            .map(|data| {
                Response::build()
                    .sized_body(data.len(), Cursor::new(data))
                    .finalize()
            })
            .map_err(|_| Status {
                code: SERIALIZER_ERROR,
            })
    }
}
