use crate::response::BasicResponse;

/// The reponse type used by `api/v1/hello/<name>`
pub type HelloWorldResponse = BasicResponse<String>;
