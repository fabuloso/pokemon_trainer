pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle(&self, command: Command) -> CommandResult {
        match command {
            Command::CatchPokemon(cmd) => CommandResult {
                message: format!("{} {}", cmd.name, cmd.number),
            },
            Command::GainExperience() => CommandResult {
                message: "exp up".to_string(),
            },
        }
    }
}
pub struct CommandResult {
    pub message: String,
}

pub struct CapturePokemon {
    pub name: String,
    pub number: u16,
}
pub enum Command {
    CatchPokemon(CapturePokemon),
    GainExperience(),
}
