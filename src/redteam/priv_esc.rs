pub fn suggest_priv_esc_paths() -> Vec<String> {
    vec![
        "Check for SUID binaries".to_string(),
        "Kernel exploit: CVE-2021-4034 (pwnkit)".to_string(),
        "Weak service permissions on Linux/Windows".to_string()
    ]
}
