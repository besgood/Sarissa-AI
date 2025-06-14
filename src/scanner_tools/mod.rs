pub mod ffuf;
pub mod amass;
pub mod httpx;
pub mod nuclei;

use ffuf::run_ffuf;
use amass::run_amass;
use httpx::run_httpx;
use nuclei::run_nuclei;

/// Trait for all scanner tools
pub trait ScanTool {
    fn name(&self) -> &str;
    fn run(&self) -> Vec<String>;
}

pub struct FFUFScan {
    pub url: String,
    pub wordlist: String,
}

impl ScanTool for FFUFScan {
    fn name(&self) -> &str {
        "ffuf"
    }

    fn run(&self) -> Vec<String> {
        run_ffuf(&self.url, &self.wordlist)
    }
}

pub struct AmassScan {
    pub domain: String,
}

impl ScanTool for AmassScan {
    fn name(&self) -> &str {
        "amass"
    }

    fn run(&self) -> Vec<String> {
        run_amass(&self.domain)
    }
}

pub struct HttpxScan {
    pub urls: Vec<String>,
}

impl ScanTool for HttpxScan {
    fn name(&self) -> &str {
        "httpx"
    }

    fn run(&self) -> Vec<String> {
        run_httpx(&self.urls)
    }
}

pub struct NucleiScan {
    pub urls: Vec<String>,
}

impl ScanTool for NucleiScan {
    fn name(&self) -> &str {
        "nuclei"
    }

    fn run(&self) -> Vec<String> {
        run_nuclei(&self.urls)
    }
}
