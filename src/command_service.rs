use std::collections::HashMap;
use std::convert::TryFrom;

use crate::command::CapturePokemon;
use crate::command::Command;
use crate::command::CommandHandler;
use crate::command::CommandResult;
use serde::Deserialize;
pub struct CommandService {}

impl CommandService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, payload: Payload) -> CommandResult {
        let handler = CommandHandler {};
        handler.handle(Command::try_from(&payload).unwrap())
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
            "expUp" => Command::GainExperience(),
            _ => unimplemented!(),
        };
        Ok(command)
    }
}
