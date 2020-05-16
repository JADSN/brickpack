use serde::{Deserialize, Serialize};

pub mod dispatcher;
pub mod maintenance_mode;
pub mod main_index;
pub mod check_auth;

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusMessage {
    Saved,
    InvalidInput(String),
    NotImplemented,
    UnderMaintenance,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyResponse {
    pub status: StatusMessage,
}




