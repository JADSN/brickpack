use crate::global_state::State;
use http_types::{Response, StatusCode};
use tide::Request;

pub async fn dispatcher(mut request: Request<State>) -> Result<Response, Response> {
    let body = match request.body_string().await {
        Ok(body) => {
            if !body.is_empty() {
                Some(body)
            } else {
                None
            }
        }
        Err(error) => {
            eprintln!("ERROR: {}", error);
            None
        }
    };

    let endpoint: String = match request.param("endpoint") {
        Ok(endpoint) => endpoint,
        Err(_) => "".to_string(),
    };

    if let Some(handler) = request.state().brickpack.get_handler(endpoint) {
        Ok(handler(body))
    } else {
        Err(Response::new(StatusCode::NotFound))
    }
}
