use std::fs::{create_dir_all, File};
use std::io::Write;
use chrono::Local;

/// Generates CSV and HTML reports from the scan and AI analysis
pub fn generate(results: &[String], ai_response: &str) {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let report_dir = "reports";
    let _ = create_dir_all(report_dir);

    let csv_path = format!("{}/report_{}.csv", report_dir, timestamp);
    let html_path = format!("{}/report_{}.html", report_dir, timestamp);

    // CSV Report
    if let Ok(mut file) = File::create(&csv_path) {
        let _ = writeln!(file, "Tool,Output");
        for section in results {
            let lines = section.lines().collect::<Vec<_>>();
            if !lines.is_empty() {
                let header = lines[0];
                let body = lines[1..].join(" ").replace(',', ";");
                let _ = writeln!(file, "\"{}\",\"{}\"", header, body);
            }
        }
        let _ = writeln!(file, "\"AI Suggestion\",\"{}\"", ai_response.replace(',', ";"));
        println!("[✓] CSV report saved to {}", csv_path);
    }

    // HTML Report
    if let Ok(mut file) = File::create(&html_path) {
        let _ = writeln!(file, "<html><head><title>Sarissa Report</title></head><body>");
        let _ = writeln!(file, "<h1>Scan Report - {}</h1>", timestamp);
        for section in results {
            let lines = section.lines().collect::<Vec<_>>();
            if !lines.is_empty() {
                let header = lines[0];
                let body = lines[1..].join("<br>");
                let _ = writeln!(file, "<h2>{}</h2><pre>{}</pre>", header, body);
            }
        }
        let _ = writeln!(file, "<h2>AI Recommendation</h2><p>{}</p>", ai_response);
        let _ = writeln!(file, "</body></html>");
        println!("[✓] HTML report saved to {}", html_path);
    }
}
