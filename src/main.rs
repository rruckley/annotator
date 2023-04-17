mod kafka;
mod grafana;
mod config;
mod annotation;
mod event;
mod correlate;

use log::{info,error};
use annotation::Annotation;
use correlate::Correlate;

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();
    let correlator = Correlate::new(grafana_client.clone());

    let event = kafka_client.get();
    let annotation = Annotation::from(event);
    match grafana_client.annotate(annotation) {
        Ok(r) => info!("Annotation worked: {r}"),
        Err(e) => error!("Annotation failure: {e}"),
    }
    
}
