use serde::Serialize;

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
