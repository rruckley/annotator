// Annotatoin module
use crate::event::Event;

pub struct Annotation {
    dashboard_id : Option<u32>,
    panel_id : Option<u32>,
    time : u64,
    time_end : u64,
    tags : Vec<String>,
    text : String,
}

impl From<Event> for Annotation {
    fn from(e: Event) -> Self {
        Annotation {
            dashboard_id : Some(0),
            panel_id  : Some(0),
            text : e.ticket_number.unwrap_or("No ticket".to_string()),
            time : 0,
            time_end : 0,
            tags : vec![
                "host=test".to_owned(),
                "cust=test".to_owned()
            ]
        }
    }
}