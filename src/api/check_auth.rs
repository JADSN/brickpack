use tide::{Request, Response};

use crate::in_memory_db::State;

pub async fn handler(request: Request<State>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
    // Authentication
    if is_authenticated(&request) {
        Ok(Response::new(StatusCode::Accepted))
    } else {
        Ok(Response::new(StatusCode::Unauthorized))
    }
}