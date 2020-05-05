#![warn(clippy::all)]

use std::collections::HashMap;

use crate::in_memory_db::State;

mod api;
mod auth;
mod in_memory_db;
mod maintenance;
mod routes;

#[derive(Debug, Default)]
pub struct App {
    endpoints: HashMap<String, fn(String) -> http_types::Response>,
    bind: String,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn set_bind(&mut self, bind: String) {
        self.bind = bind;
    }

    pub fn endpoint(&mut self, endpoint: String, handler: fn(String) -> http_types::Response) {
        self.endpoints.insert(endpoint, handler);
    }

    pub fn get_handler(&self, endpoint: String) -> Option<fn(String) -> http_types::Response> {
        let endpoints = self.get_endpoints();
        match endpoints.get(&endpoint) {
            Some(&handler) => Some(handler),
            None => None,
        }
    }

    fn get_endpoints(&self) -> HashMap<String,fn(String) -> http_types::Response> {
        self.endpoints.clone()
    }
}

pub fn run(brickpack_app: App) -> Result<(), std::io::Error> {
    use async_std::task;
    use tide::Server;

    task::block_on(async {
        let bind = brickpack_app.bind.clone();
        let mut app = Server::with_state(State::new(brickpack_app));
        app.at("/").get(crate::routes::main_index);
        app.at("/auth").get(crate::routes::check_auth);
        app.at("/maintenance")
            .patch(crate::api::maintenance_mode::presenter::handler);
        app.at("/api/:endpoint")
            .post(crate::api::dispatcher::presenter::handler);

        println!("Listening at: http://{}", bind);

        app.listen(bind).await?;
        std::process::exit(0);
    })
}
