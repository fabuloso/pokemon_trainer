use std::collections::HashMap;
use std::convert::TryFrom;

use crate::command::CapturePokemon;
use crate::command::Command;
use crate::command::CommandHandler;
use crate::command::CommandResult;
use crate::command::LevelUp;
use crate::pokemons::Pokemons;
use serde::Deserialize;

pub struct CommandService {
    handler: CommandHandler,
}

impl CommandService {
    pub fn new() -> Self {
        Self {
            handler: CommandHandler {
                pokemons: Pokemons::new(),
            },
        }
    }

    pub fn execute(&mut self, payload: Payload) -> CommandResult {
        self.handler.handle(Command::try_from(&payload).unwrap())
    }
}

#[derive(Deserialize)]
pub struct Payload {
    pub command: String,
    pub data: HashMap<String, String>,
}

impl TryFrom<&Payload> for Command {
    type Error = ();

    fn try_from(value: &Payload) -> Result<Self, Self::Error> {
        let command = match value.command.as_str() {
            "capture" => Command::CatchPokemon(CapturePokemon {
                name: value.data["name"].clone(),
                number: value.data["number"].parse::<u16>().unwrap(),
            }),
            "expUp" => Command::GainExperience(LevelUp {
                id: value.data["id"].parse::<u32>().unwrap(),
            }),
            _ => unimplemented!(),
        };
        Ok(command)
    }
}
