// Annotatoin module

pub struct Annotation {
    dashboard_id : Option<u32>,
    panel_id : Option<u32>,
    time : u64,
    time_end : u64,
    tags : Vec<String>,
    text : String,
}