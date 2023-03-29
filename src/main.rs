#[macro_use]
extern crate env_logger;

mod kafka;
mod grafana;

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();
}
