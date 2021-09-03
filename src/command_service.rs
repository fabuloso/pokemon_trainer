use std::collections::HashMap;
use std::convert::TryFrom;

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
