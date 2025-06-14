use chrono::Local;

/// Returns a timestamp string in format: YYYYMMDD_HHMMSS
pub fn timestamp() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Cleans command output by trimming and normalizing whitespace
pub fn clean_output(output: &str) -> String {
    output.trim().replace("\r", "").replace("\n\n", "\n")
}

/// Extracts URLs or IPs from scanner output (basic stub)
pub fn extract_urls(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.contains("http://") || line.contains("https://"))
        .map(|line| line.trim().to_string())
        .collect()
}

/// Writes content to a file with basic error handling
pub fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)?;
    Ok(())
}
