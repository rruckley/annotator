// Interact with Grafan to create annotations

use log::{info};

pub struct GrafanaClient {

}

impl GrafanaClient {
    pub fn new() -> Self {
        info!("Creating new GrafanaClient()");
        Self {}
    }
}