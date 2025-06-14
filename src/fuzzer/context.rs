//! Builds prompt context from environment for AI to decide fuzz direction

use crate::scanner::ScanResult;

pub fn build_prompt_context(scan_data: &[ScanResult]) -> String {
    let mut ctx = String::new();

    for result in scan_data {
        ctx.push_str(&format!("Host: {}\nPort: {}\nProtocol: {}\nBanner: {}\n\n",
            result.ip,
            result.port,
            result.protocol,
            result.banner.as_deref().unwrap_or("N/A")
        ));
    }

    ctx.push_str("Suggest fuzzing strategies, payload categories, and request types.");
    ctx
}
