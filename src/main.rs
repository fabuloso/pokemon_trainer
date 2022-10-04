mod command;
mod command_service;
mod pokemons;

use std::sync::{Arc, Mutex};

use command_service::CommandService;
use rouille::{input::json_input, router, try_or_400};

use crate::command_service::Payload;
fn main() {
    let command_service = Arc::new(Mutex::new(CommandService::new()));
    rouille::start_server("localhost:4567", move |request| {
        router!(request,
        (POST) (/pokemons) => {
            let data : Payload = try_or_400!(json_input(request));
            let command_result = command_service.lock().unwrap().execute(data);
            rouille::Response::json(&command_result)
        },
        (GET) (/pokemons/{_id : u32}) => {
            rouille::Response::text("ciao")
        },
        _ => {
            rouille::Response::empty_400()
        })
    });
}
