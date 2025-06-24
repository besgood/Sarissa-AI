//! Evaluates response to determine success of fuzzing

use reqwest::blocking::Response;

pub fn evaluate_response(payload: &str, response: &Response) {
    let status = response.status();
    let length = response.text().unwrap_or_default().len();

    if status.is_server_error() || length > 1000 {
        println!("[!] Interesting response for payload '{}': status={} length={}", payload, status, length);
    } else if status.is_success() && length > 300 {
        println!("[~] Possible reflection or processing: '{}' ({} bytes)", payload, length);
    } else {
        println!("[-] {} => status={} length={} (benign)", payload, status, length);
    }
}
