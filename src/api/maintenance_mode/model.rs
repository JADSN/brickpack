use tide::Request;

use crate::api::StatusMessage;
use crate::global_state::State;

pub fn maintenance_mode(
    request: &Request<State>,
    mode: bool,
) -> Result<StatusMessage, StatusMessage> {
    if mode {
        request.state().maintenance_mode_on();
    } else {
        request.state().maintenance_mode_off();
    }
    Ok(StatusMessage::Saved)
}
