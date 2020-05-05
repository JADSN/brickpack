use crate::in_memory_db::State;
use http_types::{Response, StatusCode};
use tide::Request;

pub async fn dispatcher(mut request: Request<State>) -> Result<Response, Response> {
    let body = request.body_string().await.unwrap();
    let endpoint: String = match request.param("endpoint") {
        Ok(endpoint) => endpoint,
        Err(_) => "".to_string()
    };

    if let Some(handler) = request.state().brickpack.get_handler(endpoint) {
        Ok(handler(body))
    } else {
        Err(Response::new(StatusCode::NotFound))
    }
}
