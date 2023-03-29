// Interact with Grafan to create annotations

use log::{info};
use crate::config::Config;

pub struct GrafanaClient {

}

impl GrafanaClient {
    pub fn new() -> Self {
        let url = Config::get("GRAFANA_URL");
        info!("Creating new GrafanaClient({url})");
        Self {}
    }
}