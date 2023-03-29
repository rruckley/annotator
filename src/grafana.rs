// Interact with Grafan to create annotations

use log::{info,debug};
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

    pub fn annotate(&self,_annotation : Annotation) -> Result<String,String> {
        // Create supplied annotation in Grafana
        debug!("Creating annotation in Grafana");
        Err("Create not implemention}".to_owned())
    }
}