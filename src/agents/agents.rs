use super::task_bus::Task;

pub fn handle_task(task: &Task) -> String {
    match task.name.as_str() {
        "ExploitAgent" => format!("[ExploitAgent] Executing: {}", task.context),
        "ReconAgent" => format!("[ReconAgent] Scanning deeper: {}", task.context),
        "ReportAgent" => format!("[ReportAgent] Adding insights to report: {}", task.context),
        _ => format!("[Unknown Agent] Task ignored: {:?}", task),
    }
}
