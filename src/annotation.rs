// Annotatoin module
use crate::event::Event;

pub struct Annotation {
    pub dashboard_id : Option<u32>,
    pub panel_id : Option<u32>,
    pub time : u64,
    pub time_end : u64,
    pub tags : Vec<String>,
    pub text : String,
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
                "cust=test".to_owned(),
            ]
        }
    }
}