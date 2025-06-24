
#[derive(Debug)]
pub struct ReActLog {
    pub step: String,
}

#[derive(Debug)]
pub struct ThoughtStep {
    pub id: u32,
    pub content: String,
}

pub fn log_trace_event(category: &str, message: &str) {
    println!("[TRACE] [{}] {}", category, message);
}
