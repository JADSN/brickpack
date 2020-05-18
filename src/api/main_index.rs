use http_types::StatusCode;
use tide::{Request, Response};

use crate::auth::is_in_maintenance_mode;
use crate::global_state::State;

pub async fn handler(request: Request<State>) -> tide::Result {
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
        Ok(Response::new(StatusCode::Found))
    } else {
        Ok(Response::new(StatusCode::ServiceUnavailable))
    }
}
