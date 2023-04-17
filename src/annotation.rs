// Annotatoin module
use crate::event::Event;

pub struct Annotation {
    pub dashboard_id : Option<u32>,
    pub panel_id : Option<u32>,
    pub time : u64,
    pub time_end : Option<u64>,
    pub tags : Vec<String>,
    pub text : String,
}

impl From<Event> for Annotation {
    fn from(e: Event) -> Self {
        // Need to create tags representing customer and service
        let customer = e.customer;
        let service = e.service;
        let timestamp = e.start_time;
        let time_end = e.finish_time.map(|t| 0);
        Annotation {
            dashboard_id : Some(0),
            panel_id  : Some(0),
            text : e.ticket_number.unwrap_or("No ticket".to_string()),
            time : 0,
            time_end : time_end,
            tags : vec![
                format!("host={}",service),
                format!("customer={}",customer),
            ]
        }
    }
}