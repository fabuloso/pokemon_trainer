mod command;
mod command_service;
mod pokemons;

use command_service::CommandService;
use rouille::{input::json_input, router, try_or_400};

use crate::command_service::Payload;
fn main() {
    let command_service = CommandService::new();
    rouille::start_server("localhost:4567", move |request| {
        router!(request,
        (POST)(/pokemons) => {
            let data : Payload = try_or_400!(json_input(request));
            let command_result = command_service.execute(data);
            rouille::Response::json(&command_result)
        },
        _ => {
            rouille::Response::empty_400()
        })
    });
}
