mod kafka;
mod grafana;

use log::{info};

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();
}
