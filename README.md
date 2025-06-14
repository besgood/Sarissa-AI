# Sarissa-AI

**Sarissa-AI** is a modular, AI-powered offensive security automation framework built in Rust. It integrates traditional scanning tools with local AI reasoning to support advanced penetration testing workflows — from reconnaissance and fuzzing to exploitation and multi-agent coordination.

> “Intelligence-guided offensive security automation.”

---

## 🚀 Key Features

### 🔍 Scanning & Enumeration
- ✅ **Naabu**, **Nmap**, and **Nuclei** integration for port and service discovery
- 🌐 Web enumeration with **HTTPX**, **Amass**, **FFUF**, **Gobuster**, and **Nikto**
- 📄 Target list support via `targets.txt`
- ⚙️ Port selection flags: `--ports`, `--top-ports`

### 🧠 Local AI Reasoning
- 🤖 LLM-driven scan analysis, recommendations, and feedback loop
- 📝 Local JSONL feedback log for model refinement (`ai_feedback_log.jsonl`)
- 🧠 Uses prompts to generate follow-up actions based on real scan context

### 🛠️ Exploitation Support
- 🎯 CVE-aware exploit module (PoC-focused)
- 📑 Exploit task suggestions from AI
- 🛠️ Integration-ready with SQLMap, WPScan, etc.

### 🔄 Multi-Agent Framework
- 🧩 Agents for `recon`, `fuzz`, `exploit`, `report`, coordinated via shared memory
- 🧠 Central **TaskCoordinator** and **AgentMemory**
- 🤝 Modular decision-making and memory passing

### 🧪 Fuzzing Engine
- 🧬 Mutation-based engine using payload categories (XSS, SQLi, LFI, SSRF, Path)
- 🎯 Targeted endpoint fuzzing with `--fuzz` and `--category` flags

### 📡 Proxy & Web Application Testing
- 🧱 **Burp Suite** request log ingestion (`--burp burp_log.txt`)
- 🔐 Auth handling, replay support, endpoint strategy planning

### 🔁 Integrations
- 📄 **Nessus CSV** import and AI-driven result triage
- 💣 Manual suggestion execution: SQLMap, Gobuster, Nikto, etc.

### 📊 Reporting
- 🧾 HTML, CSV, and JSON reporting with AI annotations
- 📌 Integrated exploit and agent summaries

---

## 🧪 Usage

```bash
sarissa <targets.txt> <wordlist_path> [OPTIONS]
