
use rand::seq::SliceRandom;
use serde::{Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
pub struct PhishingPretext {
    pub target_name: String,
    pub lure_type: String,
    pub email_subject: String,
    pub email_body: String,
    pub fake_domain: String,
}

pub fn generate_phishing_pretext(target_name: &str) -> PhishingPretext {
    let lures = vec![
        "Security Alert",
        "Payment Failure",
        "Account Locked",
        "New Voicemail",
        "Urgent Action Required",
        "IT Helpdesk Notice"
    ];

    let domains = vec![
        "support-paypal.com",
        "secure-microsoft-login.com",
        "voicemail-alert.com",
        "it-helpdesk-secure.com",
        "company-notice.net"
    ];

    let lure_type = lures.choose(&mut rand::thread_rng()).unwrap_or(&"Important Notice");
    let fake_domain = domains.choose(&mut rand::thread_rng()).unwrap_or(&"secure-update.com");

    let email_subject = format!("[{}] Please verify your account", lure_type);
    let email_body = format!(
        "Dear {},\n\nWe noticed unusual activity in your account. Please verify your identity by visiting the secure link below:\n\nhttps://{}/verify\n\nThank you,\nSecurity Team",
        target_name, fake_domain
    );

    PhishingPretext {
        target_name: target_name.to_string(),
        lure_type: lure_type.to_string(),
        email_subject,
        email_body,
        fake_domain: fake_domain.to_string(),
    }
}

pub fn save_pretext(pretext: &PhishingPretext) {
    let path = format!("pretext_{}.json", pretext.target_name.to_lowercase());
    let mut file = File::create(&path).expect("Unable to create file");
    let content = serde_json::to_string_pretty(pretext).unwrap();
    file.write_all(content.as_bytes()).expect("Failed to write");
}
