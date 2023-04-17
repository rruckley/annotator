// Standard event payload
use crate::grafana::GrafanaPanel;

#[derive(Clone)]
pub struct Event {
    pub id : String,
    source : String,
    r#type : String,
    sub_type: String,
    severity : String,
    start_time : String,
    finish_time : Option<String>,
    message : String,
    pub customer : String,
    customer_uid : Option<u16>,
    pub service : String,
    pub ticket_number : Option<String>,
    details : Option<String>,
}

pub struct EnrichedEvent {
    pub event : Event,
    pub panel : GrafanaPanel,
}

impl std::default::Default for Event {
    fn default() -> Event {
        Event {
            id : "sdf0-23034-abcnd-023490".to_owned(),
            source : "RUST".to_owned(),
            r#type : "Test".to_owned(),
            sub_type : "Test".to_owned(),
            severity : "INFO".to_owned(),
            start_time : "01-01-2023".to_owned(),
            finish_time: Some("01-01-2023".to_owned()),
            message : "Sample messsage".to_owned(),
            customer : "EBC".to_owned(),
            customer_uid : Some(1234),
            service : "3800test".to_owned(),
            ticket_number : Some("OBCS123489".to_owned()),
            details : Some("{ }".to_owned()),

        }
    }
}