use tide::{Request, Response};

use crate::in_memory_db::State;

pub async fn main_index(request: Request<State>) -> tide::Result {
    use crate::auth::{is_authenticated, is_in_maintenance_mode};
    use http_types::StatusCode;
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
        // Authentication
        if is_authenticated(&request) {
            Ok(Response::new(StatusCode::Found))
        } else {
            Ok(Response::new(StatusCode::Unauthorized))
        }
    } else {
        Ok(Response::new(StatusCode::ServiceUnavailable))
    }
}

pub async fn check_auth(request: Request<State>) -> tide::Result {
    use crate::auth::is_authenticated;
    use http_types::StatusCode;
        // Authentication
        if is_authenticated(&request) {
            Ok(Response::new(StatusCode::Accepted))
        } else {
            Ok(Response::new(StatusCode::Unauthorized))
        }
}
