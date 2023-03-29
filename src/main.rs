mod kafka;
mod grafana;
mod config;
mod annotation;
mod event;

use log::{info};

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();
}
