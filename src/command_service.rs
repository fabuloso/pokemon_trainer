use crate::command::{
    CapturePokemonCommandHandler, Command, CommandHandler, CommandResult, CommandType,
};
use serde::Deserialize;
pub struct CommandService {}

impl CommandService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, payload: Payload) -> CommandResult {
        let handler = CapturePokemonCommandHandler {};
        handler.handle(Command {
            command_type: CommandType::from(payload.command),
        })
    }
}
#[derive(Deserialize)]
pub struct Payload {
    command: String,
}
