pub trait CommandHandler {
    fn handle(&self, command: Command) -> CommandResult;
}

pub struct Command {
    pub command_type: CommandType,
}

pub struct CommandResult {
    pub message: String,
}

pub enum CommandType {
    CapturePokemon,
}

impl From<String> for CommandType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "capture" => Self::CapturePokemon,
            _ => unimplemented!(),
        }
    }
}
