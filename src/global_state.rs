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
    pub fn maintenance_mode_on(&self) {
        let mut maintenance_mode = self.maintenance_mode.write().unwrap();
        *maintenance_mode = true;
    }

    pub fn maintenance_mode_off(&self) {
        let mut maintenance_mode = self.maintenance_mode.write().unwrap();
        *maintenance_mode = false;
    }

    pub fn get_maintenance_mode(&self) -> bool {
        let maintenance_mode = self.maintenance_mode.read().unwrap();
        *maintenance_mode
    }
}


