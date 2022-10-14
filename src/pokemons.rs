use rdkafka::{producer::BaseProducer, ClientConfig};

use crate::{event_bus::KafkaEventBus, EventBus};

pub struct Pokemons {
    pub pokemons: Vec<Pokemon>,
    event_bus: KafkaEventBus,
}

impl Pokemons {
    pub fn new() -> Self {
        let producer: BaseProducer = ClientConfig::new()
            .set("bootstrap.servers", "kafka")
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");
        Self {
            pokemons: vec![],
            event_bus: KafkaEventBus {
                publisher: producer,
            },
        }
    }

    pub fn add(&mut self, pokemon: Pokemon) -> u32 {
        self.pokemons.push(pokemon);
        self.pokemons.len() as u32
    }

    pub fn by_name(&self, name: String) -> Option<Pokemon> {
        self.pokemons
            .iter()
            .find(|&pokemon| name == pokemon.name)
            .cloned()
    }

    pub fn save(&self, pokemon: Pokemon) {
        for event in pokemon.events() {
            self.event_bus.publish(event);
        }
    }
}

#[derive(Clone)]
pub struct Pokemon {
    pub name: String,
    pub number: u16,
    pub level: u8,
    pub exp: u16,
    events: Vec<Box<Event>>,
}

impl Pokemon {
    pub fn events(self) -> Vec<Box<Event>> {
        self.events
    }
}

impl Pokemon {
    fn apply(self, event: Box<Event>) {}

    pub fn new(name: String, number: u16) -> Self {
        Self {
            name: name.to_string(),
            number,
            level: 1,
            exp: 0,
            events: vec![Box::new(Event::PokemonCaptured(name, number))],
        }
    }
}

#[derive(Clone)]
pub enum Event {
    PokemonCaptured(String, u16),
}
