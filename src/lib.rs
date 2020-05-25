#![warn(clippy::all)]
#![recursion_limit="256"]

mod api;
mod auth;
mod global_state;
pub mod http_client;

use crate::global_state::State;
use async_std::task;
use http_types::Response;
use std::collections::HashMap;
use tide::Server;

#[derive(Debug, Default)]
pub struct App {
    endpoints: HashMap<String, fn(Option<String>) -> Response>,
    listen: String,
}

impl App {
    // * Public methods

    pub fn new() -> App {
        App::default()
    }

    pub fn listen(&mut self, listen: String) {
        self.listen = listen;
    }

    pub fn add_endpoint(&mut self, endpoint: &str, handler: fn(Option<String>) -> Response) {
        self.endpoints.insert(endpoint.to_string(), handler);
    }

    pub fn get_handler(&self, endpoint: String) -> Option<fn(Option<String>) -> Response> {
        let endpoints = self.get_endpoints();
        match endpoints.get(&endpoint) {
            Some(&handler) => Some(handler),
            None => None,
        }
    }

    pub fn run(self) -> Result<(), std::io::Error> {
        const DEFAULT_LISTEN: &str = "127.0.0.1:8000";
        task::block_on(async {
            let listen = if self.get_listen().is_empty() {
                DEFAULT_LISTEN.to_string()
            } else {
                self.get_listen()
            };
            self.startup_message();
            let mut app = Server::with_state(State::new(self));
            app.at("/").get(crate::api::main_index::handler);
            app.at("/auth").get(crate::api::check_auth::handler);
            app.at("/maintenance")
                .patch(crate::api::maintenance_mode::presenter::handler);
            app.at("/api/:endpoint")
                .post(crate::api::dispatcher::presenter::handler);
            match crate::auth::get_token_from_env() {
                Some(token) => {
                    println!();
                    println!("CLIENT_TOKEN: {}", token);
                    println!();
                    println!("Listening at: http://{}", listen);
                    app.listen(listen).await?;
                    std::process::exit(0);
                }
                None => {
                    std::process::exit(1);
                }
            }
        })
    }

    // * Private methods

    fn get_listen(&self) -> String {
        self.listen.clone()
    }

    fn get_endpoints(&self) -> HashMap<String, fn(Option<String>) -> Response> {
        self.endpoints.clone()
    }

    fn get_serialized_endpoints(&self) -> Vec<String> {
        let mut endpoints: Vec<String> = vec![];
        for endpoint in self.get_endpoints().keys() {
            endpoints.push(endpoint.to_string());
        }
        endpoints
    }

    fn startup_message(&self) {
        let endpoints = self.get_serialized_endpoints();
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
}
