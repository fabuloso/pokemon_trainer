use crate::pokemons::{Pokemon, Pokemons};

use super::{Command, CommandResult};

pub struct CommandHandler {
    pub pokemons: Pokemons,
}

impl CommandHandler {
    pub fn handle(&mut self, command: Command) -> CommandResult {
        match command {
            Command::CatchPokemon(cmd) => match self.pokemons.by_name(cmd.name.clone()) {
                Some(_) => CommandResult {
                    id: 0,
                    message: "You already catch this kind of pokemon".into(),
                },
                None => {
                    let id = self
                        .pokemons
                        .add(Pokemon::new(cmd.name.clone(), cmd.number));

                    CommandResult {
                        id,
                        message: format!("{} {}", cmd.name, cmd.number),
                    }
                }
            },
            Command::GainExperience(cmd) => CommandResult {
                id: cmd.id,
                message: "level up".to_string(),
            },
        }
    }
}
