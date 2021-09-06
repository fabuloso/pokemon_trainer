pub struct Pokemons {
    pub pokemons: Vec<Pokemon>,
}

impl Pokemons {
    pub fn new() -> Self {
        Self { pokemons: vec![] }
    }

    pub fn add(&mut self, pokemon: Pokemon) -> u32 {
        self.pokemons.push(pokemon);
        self.pokemons.len() as u32
    }
}

pub struct Pokemon {
    pub name: String,
    pub number: u16,
    pub level: u8,
    pub exp: u16,
}

impl Pokemon {
    pub fn new(name: String, number: u16) -> Self {
        Self {
            name,
            number,
            level: 1,
            exp: 0,
        }
    }
}
