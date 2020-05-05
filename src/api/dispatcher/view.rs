use tide::Response;
use http_types::Response as InternalResponse;
pub fn dispatcher(response: InternalResponse) -> Response {
    Response::from(response)
}
