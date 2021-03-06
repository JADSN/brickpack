use tide::{Request, Response, StatusCode};

use crate::auth::token::is_in_maintenance_mode;
use crate::global_state::State;

pub async fn handler(request: Request<State>) -> tide::Result {
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
        Ok(Response::new(StatusCode::Found))
    } else {
        Ok(Response::new(StatusCode::ServiceUnavailable))
    }
}
