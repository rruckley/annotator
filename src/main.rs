mod kafka;
mod grafana;
mod config;
mod annotation;
mod event;

use log::{info};
use annotation::Annotation;

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();

    let event = kafka_client.get();
    let annotation = Annotation::from(event);
    grafana_client.annotate(annotation);
    
}
