#![warn(clippy::all)]

pub mod maintenance_mode;
pub mod dispatcher;

use serde::{Deserialize, Serialize};

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

