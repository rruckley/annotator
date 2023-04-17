mod kafka;
mod grafana;
mod config;
mod annotation;
mod event;
mod correlate;

use log::{info,debug,error};
use annotation::Annotation;
use correlate::Correlate;

fn main() {
    env_logger::init();
    info!("Grafana Annotator");

    let kafka_client = kafka::KafkaClient::new();
    let grafana_client = grafana::GrafanaClient::new();
    let correlator = Correlate::new(grafana_client.clone());

    let event = kafka_client.get();
    let panel = correlator.find_panel(&event);

    match panel {
        Some(p) => {
            debug!("Found a panel")
        },
        None => {
            debug!("No panel found");
        }
    }
    //let annotation = Annotation::from(event);
  
    
}
