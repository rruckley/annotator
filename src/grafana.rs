// Interact with Grafan to create annotations

use log::{info};
use crate::config::Config;
use crate::annotation::Annotation;

pub struct GrafanaClient {

}

impl GrafanaClient {
    pub fn new() -> Self {
        let url = Config::get("GRAFANA_URL");
        info!("Creating new GrafanaClient({url})");
        Self {}
    }

    pub fn annotate(annotation : Annotation) -> Result<String,String> {
        // Create supplied annotation in Grafana
        Err("Not implemented".to_owned())
    }
}