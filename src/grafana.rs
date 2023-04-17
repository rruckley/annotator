// Interact with Grafan to create annotations

use log::{info,debug};
use crate::config::Config;
use crate::annotation::Annotation;

#[derive(Clone)]
pub struct GrafanaClient {
}

pub struct GrafanaPanel {
    pub dashboard : String,
    pub panel : Option<String>,
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
        Err("Create not implemeted".to_owned())
    }

    pub fn get_org_by_name(&self,org_name : String) -> Option<String> {
        None
    }
}