//! Generates input mutations for fuzzing

pub fn generate_mutations(base: &[&str]) -> Vec<String> {
    let mut results = Vec::new();

    for &item in base {
        results.push(item.to_string());
        results.push(format!("{}%00", item)); // Null byte
        results.push(format!("{}'\"`", item)); // Quote injection
        results.push(format!("<{}>", item));  // HTML tags
        results.push(format!("..//{}//..", item)); // Path traversal variants
        results.push(format!("{}--", item));  // SQL comment style
    }

    results.sort();
    results.dedup();
    results
}
