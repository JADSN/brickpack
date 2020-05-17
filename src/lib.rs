#![warn(clippy::all)]

use std::collections::HashMap;

use crate::in_memory_db::State;

mod api;
mod auth;
pub mod http_client;
mod in_memory_db;
mod maintenance;

#[derive(Debug, Default)]
pub struct App {
    endpoints: HashMap<String, fn(Option<String>) -> http_types::Response>,
    bind: String,
}

impl App {
    pub fn new() -> App {
        App::default()
    }

    pub fn set_bind(&mut self, bind: String) {
        self.bind = bind;
    }

    pub fn add_endpoint(
        &mut self,
        endpoint: &str,
        handler: fn(Option<String>) -> http_types::Response,
    ) {
        self.endpoints.insert(endpoint.to_string(), handler);
    }

    pub fn get_handler(
        &self,
        endpoint: String,
    ) -> Option<fn(Option<String>) -> http_types::Response> {
        let endpoints = self.get_endpoints();
        match endpoints.get(&endpoint) {
            Some(&handler) => Some(handler),
            None => None,
        }
    }

    fn get_endpoints(&self) -> HashMap<String, fn(Option<String>) -> http_types::Response> {
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
    println!();
    println!("Brickpack Web Framework v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("System Endpoints:");
    println!("                       GET   - /");
    println!("                       GET   - /auth");
    println!("                       PATCH - /maintenance");
    println!();
    if !endpoints.is_empty() {
        println!("Application Endpoints:");
        for endpoint in endpoints {
            println!("                       POST  - /api/{}", endpoint)
        }
        println!();
    }
}

pub fn run(brickpack_app: App) -> Result<(), std::io::Error> {
    use async_std::task;
    use tide::Server;
    const DEFAULT_LISTEN: &str = "127.0.0.1:8000";

    task::block_on(async {
        let mut bind = brickpack_app.bind.clone();
        if bind.is_empty() {
            bind = DEFAULT_LISTEN.to_string();
        }
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
                std::process::exit(1);
            }
        }
    })
}
