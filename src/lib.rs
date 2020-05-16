#![warn(clippy::all)]

use std::collections::HashMap;

use crate::in_memory_db::State;

mod api;
mod auth;
mod in_memory_db;
mod maintenance;

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

    pub fn add_endpoint(&mut self, endpoint: &str, handler: fn(String) -> http_types::Response) {
        self.endpoints.insert(endpoint.to_string(), handler);
    }

    pub fn get_handler(&self, endpoint: String) -> Option<fn(String) -> http_types::Response> {
        let endpoints = self.get_endpoints();
        match endpoints.get(&endpoint) {
            Some(&handler) => Some(handler),
            None => None,
        }
    }

    fn get_endpoints(&self) -> HashMap<String, fn(String) -> http_types::Response> {
        self.endpoints.clone()
    }

    pub fn get_serialized_endpoints(&self) -> Vec<String> {
        let mut endpoints: Vec<String> = vec![];
        for endpoint in self.get_endpoints().keys() {
            endpoints.push(endpoint.to_string());
        }
        endpoints
    }
}

fn show_endpoints(endpoints: Vec<String>) {
    println!("Loading Brickpack...");
    println!();
    println!("Brickpack Web Framework v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Service Routes:");
    println!("                      GET   - /");
    println!("                      GET   - /auth");
    println!("                      PATCH - /maintenance");
    println!();
    println!("Custom Routes:");
    for endpoint in endpoints {
        println!("                      POST  - /api/{}", endpoint)
    }
    println!();
}

pub fn run(brickpack_app: App) -> Result<(), std::io::Error> {
    use async_std::task;
    use tide::Server;

    task::block_on(async {
        let bind = brickpack_app.bind.clone();
        let endpoints = brickpack_app.get_serialized_endpoints();
        let mut app = Server::with_state(State::new(brickpack_app));
        app.at("/").get(crate::api::main_index::handler);
        app.at("/auth").get(crate::api::check_auth::handler);
        app.at("/maintenance")
            .patch(crate::api::maintenance_mode::presenter::handler);
        app.at("/api/:endpoint")
            .post(crate::api::dispatcher::presenter::handler);

        match crate::auth::get_token_from_env() {
            Some(token) => {
                show_endpoints(endpoints);
                println!();
                println!("CLIENT_TOKEN: {}", token);
                println!();
                println!("Listening at: http://{}", bind);
                app.listen(bind).await?;
                std::process::exit(0);
            }
            None => {
                // app.listen(bind).await?;
                std::process::exit(1);
            }
        }
    })
}
