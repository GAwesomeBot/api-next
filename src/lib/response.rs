use serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    err: Option<String>,
    data: Option<T>,
}

pub fn respond<T>(data: Option<T>, err: Option<String>) -> Json<Response<T>> {
    let response = Response {
        err,
        data,
    };
    Json(response)
}
