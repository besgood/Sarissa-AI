use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TargetProfile {
    pub ip: String,
    pub os: Option<String>,
    pub open_ports: Vec<u16>,
    pub services: HashMap<u16, String>,
    pub web_tech: Vec<String>,
}

impl TargetProfile {
    pub fn new(ip: &str) -> Self {
        TargetProfile {
            ip: ip.to_string(),
            os: None,
            open_ports: vec![],
            services: HashMap::new(),
            web_tech: vec![],
        }
    }

    pub fn add_port(&mut self, port: u16, service: &str) {
        self.open_ports.push(port);
        self.services.insert(port, service.to_string());
    }

    pub fn set_os(&mut self, os: &str) {
        self.os = Some(os.to_string());
    }

    pub fn add_web_tech(&mut self, tech: &str) {
        self.web_tech.push(tech.to_string());
    }

    pub fn summarize(&self) -> String {
        format!(
            "Target: {}\nOS: {}\nPorts: {:?}\nServices: {:?}\nWeb Tech: {:?}",
            self.ip,
            self.os.clone().unwrap_or("Unknown".into()),
            self.open_ports,
            self.services,
            self.web_tech
        )
    }
}
