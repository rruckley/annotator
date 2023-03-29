// Pull notifications from Kafka topic
use log::{info};
use crate::config::Config;
pub struct KafkaClient {

}

impl KafkaClient {
    pub fn new() -> Self {
        let url = Config::get("KAFKA_URL");
        info!("Creating new KafkaClient({url})");
        Self {}
    }
}