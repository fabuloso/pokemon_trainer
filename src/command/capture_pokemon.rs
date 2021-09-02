use super::{command::CommandHandler, CommandResult};

pub struct CapturePokemonCommandHandler {}

impl CommandHandler for CapturePokemonCommandHandler {
    fn handle(&self, command: super::Command) -> super::CommandResult {
        CommandResult {
            message: "Bella li".to_string(),
        }
    }
}
