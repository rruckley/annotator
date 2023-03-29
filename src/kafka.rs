// Pull notifications from Kafka topic
use log::{info};
pub struct KafkaClient {

}

impl KafkaClient {
    pub fn new() -> Self {
        info!("Creating new KafkaClient()");
        Self {}
    }
}