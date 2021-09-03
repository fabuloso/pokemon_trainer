mod command;
mod command_service;
use std::convert::TryFrom;

use command::{CapturePokemon, Command};
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
            rouille::Response::text(command_result.message)
        },
        _ => {
            rouille::Response::empty_400()
        })
    });
}

impl TryFrom<&Payload> for Command {
    type Error = ();

    fn try_from(value: &Payload) -> Result<Self, Self::Error> {
        let command = match value.command.as_str() {
            "capture" => Command::CatchPokemon(CapturePokemon {
                name: value.data["name"].clone(),
                number: value.data["number"].parse::<u16>().unwrap(),
            }),
            "expUp" => Command::GainExperience(),
            _ => unimplemented!(),
        };
        Ok(command)
    }
}
