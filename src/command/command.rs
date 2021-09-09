use serde::Serialize;

use crate::pokemons::{Pokemon, Pokemons};

pub struct CommandHandler {
    pub pokemons: Pokemons,
}

impl CommandHandler {
    pub fn handle(&mut self, command: Command) -> CommandResult {
        match command {
            Command::CatchPokemon(cmd) => {
                let id = self
                    .pokemons
                    .add(Pokemon::new(cmd.name.clone(), cmd.number.clone()));
                CommandResult {
                    id,
                    message: format!("{} {}", cmd.name, cmd.number),
                }
            }
            Command::GainExperience(cmd) => CommandResult {
                id: cmd.id,
                message: "level up".to_string(),
            },
        }
    }
}
#[derive(Serialize)]
pub struct CommandResult {
    pub id: u32,
    pub message: String,
}

pub struct CapturePokemon {
    pub name: String,
    pub number: u16,
}

pub struct LevelUp {
    pub id: u32,
}

pub enum Command {
    CatchPokemon(CapturePokemon),
    GainExperience(LevelUp),
}
