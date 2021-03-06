use serde::Deserialize;
use tide::{Error, Request, Response, StatusCode};

use crate::auth::token::is_authenticated;
use crate::global_state::State;

use super::model;
use super::view;

#[derive(Deserialize, Debug)]
struct Maintenance {
    mode: bool,
}

pub async fn handler(mut request: Request<State>) -> tide::Result {
    // TODO: dbg!(&request.peer_addr());
    // TODO: Run only if source IP is 127.0.0.1
    // Authentication:
    if is_authenticated(&request) {
        let maintenance: Maintenance = match request.body_json().await {
            Ok(data) => data,
            Err(error) => {
                return Err(Error::from_str(
                    StatusCode::BadRequest,
                    format!("Invalid body -> Err({})", error),
                ))
            }
        };
        // Model
        match model::maintenance_mode(&request, maintenance.mode) {
            Ok(model) => {
                // View
                let view = view::maintenance_mode(model);
                Ok(view)
            }
            Err(error) => Ok(Response::new(StatusCode::Ok).body_json(&error).unwrap()),
        }
    } else {
        Err(Error::from_str(StatusCode::Unauthorized, "Access Denied"))
    }
}
