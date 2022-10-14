use async_trait::async_trait;
use rdkafka::producer::{BaseProducer, BaseRecord};

use crate::pokemons::Event;

#[async_trait]
pub trait EventBus {
    async fn publish(&self, event: Box<Event>);
}

pub struct KafkaEventBus {
    pub publisher: BaseProducer,
}

impl KafkaEventBus {}

#[async_trait]
impl EventBus for KafkaEventBus {
    async fn publish(&self, event: Box<Event>) {
        let payload = "".to_string();
        let event = "".to_string();
        let record = BaseRecord::to("event_bus").payload(&payload).key(&event);
        let _ = self.publisher.send(record);
    }
}
