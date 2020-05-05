use std::sync::RwLock;

use crate::App;

#[derive(Debug)]
pub struct State {
    pub maintenance_mode: RwLock<bool>,
    pub brickpack: App
}

impl State {
    pub fn new(app: App) -> Self {
        State {
            maintenance_mode: RwLock::new(false),
            brickpack: app
        }
    }
}


