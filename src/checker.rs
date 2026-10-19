use colored::*;
use std::path::Path;

use crate::parser::EnvFile;

pub struct CheckResult {
    pub missing_in_example: Vec<String>,
    pub missing_in_env: Vec<String>,
    pub empty_in_env: Vec<String>,
}

impl CheckResult {
    pub fn is_success(&self) -> bool {
        self.missing_in_example.is_empty() && self.missing_in_env.is_empty()
    }
}

pub fn check_sync(env_path: &Path, example_path: &Path) -> Result<CheckResult, String> {
    if !env_path.exists() {
        return Err(format!("Source env file not found: {}", env_path.display()));
    }
    if !example_path.exists() {
        return Err(format!("Example env file not found: {}", example_path.display()));
    }

    let env_file = EnvFile::from_file(env_path)
        .map_err(|e| format!("Failed to read {}: {}", env_path.display(), e))?;
    let example_file = EnvFile::from_file(example_path)
        .map_err(|e| format!("Failed to read {}: {}", example_path.display(), e))?;

    let mut missing_in_example = Vec::new();
    let mut missing_in_env = Vec::new();
    let mut empty_in_env = Vec::new();

    // Check keys in .env that are missing in .env.example
    for key in env_file.entries.keys() {
        if !example_file.entries.contains_key(key) {
            missing_in_example.push(key.clone());
        }
    }

    // Check keys in .env.example that are missing in .env
    for key in example_file.entries.keys() {
        if !env_file.entries.contains_key(key) {
            missing_in_env.push(key.clone());
        }
    }

    // Optional: detect empty keys in .env
    for (key, entry) in &env_file.entries {
        if entry.value.trim().is_empty() {
            empty_in_env.push(key.clone());
        }
    }

    missing_in_example.sort();
    missing_in_env.sort();
    empty_in_env.sort();

    Ok(CheckResult {
        missing_in_example,
        missing_in_env,
        empty_in_env,
    })
}

pub fn render_check_result(
    env_path: &Path,
    example_path: &Path,
    result: &CheckResult,
) {
    println!("{}", "🔍 Comparing environment files...".bold().cyan());
    println!("  📄 Target:  {}", env_path.display().to_string().yellow());
    println!("  📋 Example: {}", example_path.display().to_string().yellow());
    println!();

    if result.is_success() {
        println!("{}", "✅ PERFECT SYNC!".bold().green());
        println!(
            "{}",
            "  All keys are synchronized between .env and .env.example."
                .green()
        );
        if !result.empty_in_env.is_empty() {
            println!();
            println!(
                "{}",
                format!("ℹ️  Notice: {} key(s) in .env have empty values:", result.empty_in_env.len())
                    .dimmed()
            );
            for key in &result.empty_in_env {
                println!("    {} {}", "•".dimmed(), key.dimmed());
            }
        }
        return;
    }

    println!("{}", "❌ OUT OF SYNC DETECTED!".bold().red());
    println!();

    if !result.missing_in_example.is_empty() {
        println!(
            "{} {}",
            "[+] Missing in example template:".bold().yellow(),
            format!("({} key{})", result.missing_in_example.len(), if result.missing_in_example.len() > 1 { "s" } else { "" }).dimmed()
        );
        println!("    These exist in your .env but teammates won't know they exist:");
        for key in &result.missing_in_example {
            println!("    {} {}", "+".green().bold(), key.bright_white().bold());
        }
        println!();
    }

    if !result.missing_in_env.is_empty() {
        println!(
            "{} {}",
            "[-] Missing in local .env:".bold().red(),
            format!("({} key{})", result.missing_in_env.len(), if result.missing_in_env.len() > 1 { "s" } else { "" }).dimmed()
        );
        println!("    These are required by .env.example but missing locally:");
        for key in &result.missing_in_env {
            println!("    {} {}", "-".red().bold(), key.bright_white().bold());
        }
        println!();
    }

    println!("{}", "💡 Tip: Run `envguard generate` to automatically sync your .env.example!".cyan());
}
