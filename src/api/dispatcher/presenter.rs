use tide::{Error, Request, Response, StatusCode};

use crate::api::{BodyResponse, StatusMessage};
use crate::auth::token::{is_authenticated, is_in_maintenance_mode};
use crate::global_state::State;

use super::model;
use super::view;

pub async fn handler(request: Request<State>) -> tide::Result {
    // Check Maintenance Mode
    if !is_in_maintenance_mode(&request) {
        // Authentication:
        if is_authenticated(&request) {
            // Model
            match model::dispatcher(request).await {
                Ok(model) => {
                    // View
                    let view = view::dispatcher(model);
                    Ok(view)
                }
                Err(error) => {
                    // View
                    let view = view::dispatcher(error);
                    Ok(view)
                }
            }
        } else {
            Err(Error::from_str(StatusCode::Unauthorized, "Access Denied"))
        }
    } else {
        let body_response = BodyResponse {
            status: StatusMessage::UnderMaintenance,
        };
        Ok(Response::new(StatusCode::ServiceUnavailable)
            .body_json(&body_response)
            .unwrap())
    }
}
