/// Generates prompt templates for the LLM to analyze scan results and suggest actions.

/// General scan analysis prompt
pub fn analysis_prompt(scan_summary: &str) -> String {
    format!(
        """
You are a penetration testing assistant. Analyze the following scan results and suggest next steps.
Be specific about tools to run and justify each suggestion.

Scan Data:
{}

Output your recommendations in plain text.
""",
        scan_summary
    )
}

/// Prompt template for exploiting a specific finding
pub fn exploit_prompt(vuln_detail: &str) -> String {
    format!(
        """
You are a security expert. Given this vulnerability detail, what specific commands, payloads, or tools would you run?

Details:
{}
""",
        vuln_detail
    )
}

/// Prompt for asking the LLM to summarize risk level
pub fn risk_summary_prompt(context: &str) -> String {
    format!(
        """
Based on the following information, rate the severity of the risk (1–10) and briefly justify:

{}
""",
        context
    )
}