use http_types::Response as InternalResponse;
use tide::Response;
pub fn dispatcher(response: InternalResponse) -> Response {
    // * Convert http_types:Response to tide::Response
    Response::from_res(response)
}
