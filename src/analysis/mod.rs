//! Analysis module: entry point for correlation, retesting, and risk modeling

pub mod correlation;
pub mod retesting;
pub mod risk;

/// Run all analysis tasks sequentially and return enriched insights
pub fn run_analysis(scan_results: &[String]) -> Vec<String> {
    let correlated = correlation::map_findings(scan_results);
    let retested = retesting::retest_suggestions(&correlated);
    let enriched = risk::classify_risks(&retested);
    enriched
}
