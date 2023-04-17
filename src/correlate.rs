// Correlate between an event with a service and a dashboard / panel
use crate::grafana::{GrafanaPanel, GrafanaClient};
use crate::event::{Event,EnrichedEvent};
use log::{debug,error};

pub struct Correlate {
// Hold reference to Grafana Client
    grafana : GrafanaClient,
}

impl Correlate {
    pub fn new(grafana : GrafanaClient) -> Self {
        Self {
            grafana
        }
    }
    pub fn find_panel(&self,event : &Event) -> Option<EnrichedEvent> {
        debug!("Finding panel for event {}/{}",&event.customer, event.service);
        // First step is to find the customer org in Grafana based on event customer
        let org_id = self.grafana.get_org_by_name(event.customer.clone());
        match org_id {
            Some(org) => {
                // We can continue
                let enriched = EnrichedEvent {
                    event : event.clone(),
                    panel : GrafanaPanel {
                        dashboard : "unknown".to_string(),
                        panel : None,
                    },
                };
                Some(enriched)
            },
            None => {
                error!("No org_id found for customer: {}",&event.customer);
                return None
            },
        }
    }
    fn find_customer_org(&self,customer : &str) -> Option<String> {
        debug!("Finding org for customer {customer}");
        self.grafana.get_org_by_name(customer.to_string())
    }
}