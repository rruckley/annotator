// Pull notifications from Kafka topic
use log::{info,debug};
use crate::config::Config;
use crate::event::Event;
pub struct KafkaClient {

}

impl KafkaClient {
    pub fn new() -> Self {
        let url = Config::get("KAFKA_URL");
        info!("Creating new KafkaClient({url})");
        Self {}
    }

    pub fn get(&self) -> Event {
        debug!("Getting event");
        Event::default()
    }
}