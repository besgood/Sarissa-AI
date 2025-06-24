# Sarissa-AI

**Sarissa-AI** is a Rust-based AI-enhanced offensive security automation tool. It integrates traditional scanning utilities with local LLM intelligence (via Ollama or similar), multi-agent coordination, payload evolution, and adaptive fuzzing to support advanced penetration testing on both web applications and networks.

## 🚀 Features

### ✅ Core Capabilities
- 🧠 **LLM-assisted Exploitation Suggestions** using local models
- 🔍 **Network Reconnaissance** with Nmap, Masscan, Naabu
- 🌐 **Web App Discovery** with FFUF, HTTPX, Nuclei, Amass
- ⚡ **AI-Enhanced Fuzzing Engine** with adaptive payload memory
- 🔄 **Burp Suite Proxy Log Integration** for replay and AI review
- 📄 **Nessus CSV Parser** with finding summarization
- 🧩 **Plugin Support (via registry config)** for modular tool use
- 🕵️ **ReAct-style Multi-Agent Framework**:
  - `ReconAgent`, `ExploitAgent`, `ReportAgent`
  - Uses shared memory and streaming task coordination

### 🧪 Fuzzing Support
- Payload categories: `XSS`, `SQLi`, `LFI`, `SSRF`, `Path Traversal`, `All`
- Mutations and AI-generated test payloads
- History tracking to improve success rate over time

### 🧠 AI Integration
- LLM queries for:
  - Contextual attack recommendations
  - CVE matching
  - Fuzzing mutation logic
- Feedback loop with user validation
- ReAct logging: `Thought → Action → Observation`

### 📊 Reporting
- Auto-generated reports summarizing scan results and AI suggestions
- Includes enriched CVE mapping and exploit analysis

---

## 📦 Installation

> Requires: Rust 1.70+, Ollama (or any local LLM API endpoint), common recon tools in `PATH`

```bash
git clone https://github.com/your-org/sarissa-ai
cd sarissa-ai
cargo build --release
