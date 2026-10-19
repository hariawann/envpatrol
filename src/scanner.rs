use colored::*;
use regex::Regex;
use std::path::Path;

use crate::parser::EnvFile;

pub struct LeakFinding {
    pub key: String,
    pub line_number: usize,
    pub rule_name: &'static str,
    pub severity: Severity,
    pub sample_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Severity {
    Critical,
    High,
    Warning,
}

impl Severity {
    pub fn badge(&self) -> ColoredString {
        match self {
            Severity::Critical => "CRITICAL LEAK".on_red().white().bold(),
            Severity::High => "HIGH RISK".on_bright_red().white().bold(),
            Severity::Warning => "SUSPICIOUS".on_yellow().black().bold(),
        }
    }
}

pub struct SecretRule {
    pub name: &'static str,
    pub pattern: Regex,
    pub severity: Severity,
}

pub fn get_default_rules() -> Vec<SecretRule> {
    vec![
        SecretRule {
            name: "OpenAI Secret Key",
            pattern: Regex::new(r"sk-(?:proj-)?[A-Za-z0-9_-]{20,}").unwrap(),
            severity: Severity::Critical,
        },
        SecretRule {
            name: "AWS Access Key ID",
            pattern: Regex::new(r"\bAKIA[0-9A-Z]{16}\b").unwrap(),
            severity: Severity::Critical,
        },
        SecretRule {
            name: "Stripe Secret / Restricted Key",
            pattern: Regex::new(r"\b(?:sk|rk)_(?:live|test)_[0-9a-zA-Z]{24,}\b").unwrap(),
            severity: Severity::Critical,
        },
        SecretRule {
            name: "GitHub Personal Access Token",
            pattern: Regex::new(r"\b(?:ghp|gho|ghu|ghs|ghr)_[0-9a-zA-Z]{36}\b|\bgithub_pat_[0-9a-zA-Z_]{22,}\b").unwrap(),
            severity: Severity::Critical,
        },
        SecretRule {
            name: "Google API / Firebase Key",
            pattern: Regex::new(r"\bAIza[0-9A-Za-z-_]{35}\b").unwrap(),
            severity: Severity::High,
        },
        SecretRule {
            name: "Slack Token",
            pattern: Regex::new(r"\bxox[baprs]-[0-9a-zA-Z-]{10,}\b").unwrap(),
            severity: Severity::High,
        },
        SecretRule {
            name: "Private RSA / SSH Key",
            pattern: Regex::new(r"-----BEGIN [A-Z ]*PRIVATE KEY-----").unwrap(),
            severity: Severity::Critical,
        },
        SecretRule {
            name: "JSON Web Token (JWT)",
            pattern: Regex::new(r"\beyJ[a-zA-Z0-9_-]{10,}\.eyJ[a-zA-Z0-9_-]{10,}\.[a-zA-Z0-9_-]{10,}\b").unwrap(),
            severity: Severity::High,
        },
    ]
}

fn is_placeholder(val: &str) -> bool {
    let trimmed = val.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();

    // Exact matches
    let exact_placeholders = [
        "secret", "password", "pass", "admin", "root", "test", "demo",
        "dummy", "todo", "none", "null", "undefined", "changeme", "xxx",
        "123456", "12345678", "fake", "placeholder",
    ];
    if exact_placeholders.contains(&lower.as_str()) {
        return true;
    }

    // Prefixes / wrappers
    if lower.starts_with("your_")
        || lower.starts_with("<")
        || lower.starts_with("[")
        || lower.starts_with("{")
        || lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("example")
        || lower.ends_with("example")
        || lower.contains("placeholder")
        || lower.contains("change_me")
        || lower.contains("changeme")
    {
        return true;
    }

    false
}

pub fn scan_env_file(path: &Path) -> Result<Vec<LeakFinding>, String> {
    if !path.exists() {
        return Err(format!("File to scan not found: {}", path.display()));
    }

    let env_file = EnvFile::from_file(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let rules = get_default_rules();
    let mut findings = Vec::new();

    for (_key, entry) in &env_file.entries {
        let val = &entry.value;
        if val.is_empty() || is_placeholder(val) {
            continue;
        }

        // Test explicit known regex rules
        let mut matched_rule = false;
        for rule in &rules {
            if rule.pattern.is_match(val) {
                findings.push(LeakFinding {
                    key: entry.key.clone(),
                    line_number: entry.line_number,
                    rule_name: rule.name,
                    severity: rule.severity.clone(),
                    sample_preview: mask_secret(val),
                });
                matched_rule = true;
                break;
            }
        }

        if matched_rule {
            continue;
        }

        // Generic heuristic for sensitive keys with high-length literal values
        let upper_key = entry.key.to_uppercase();
        let is_sensitive_key = upper_key.contains("SECRET")
            || upper_key.contains("PASSWORD")
            || upper_key.contains("PRIVATE")
            || upper_key.contains("API_KEY")
            || upper_key.contains("TOKEN")
            || upper_key.contains("AUTH");

        if is_sensitive_key && val.len() >= 12 && !is_placeholder(val) {
            findings.push(LeakFinding {
                key: entry.key.clone(),
                line_number: entry.line_number,
                rule_name: "Potentially Hardcoded Secret / Credential",
                severity: Severity::Warning,
            sample_preview: mask_secret(val),
        });
        }
    }

    Ok(findings)
}

fn mask_secret(val: &str) -> String {
    if val.len() <= 6 {
        return "***".to_string();
    }
    let prefix = &val[..3];
    let suffix = &val[val.len() - 3..];
    format!("{}...{}", prefix, suffix)
}

pub fn render_scan_result(target_path: &Path, findings: &[LeakFinding]) {
    println!("{}", "🛡️  Scanning for exposed secrets & API keys...".bold().cyan());
    println!("  📄 Target: {}", target_path.display().to_string().yellow());
    println!();

    if findings.is_empty() {
        println!("{}", "✅ CLEAN! No exposed secrets detected.".bold().green());
        println!("  All values in this file appear safe or properly sanitized.");
        return;
    }

    println!(
        "{} {}",
        "🚨 SECURITY ALERT!".bold().red(),
        format!("Found {} potential secret leak(s):", findings.len()).red()
    );
    println!();

    for finding in findings {
        println!(
            "  {} Line {}: {}",
            finding.severity.badge(),
            finding.line_number.to_string().cyan(),
            finding.key.bright_white().bold()
        );
        println!("     Type:    {}", finding.rule_name.yellow());
        println!("     Preview: {}", finding.sample_preview.dimmed());
        println!();
    }

    println!("{}", "⚠️  Action Required: Replace real keys with safe placeholders (e.g. 'your_key_here') before committing!".yellow().bold());
}
