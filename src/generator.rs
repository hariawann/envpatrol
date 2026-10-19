use std::fs;
use std::path::Path;

use crate::parser::{EnvFile, LineItem};

pub struct GenerateOptions {
    pub keep_defaults: bool,
    pub force: bool,
}

pub fn generate_example(
    source_path: &Path,
    output_path: &Path,
    opts: &GenerateOptions,
) -> Result<usize, String> {
    if !source_path.exists() {
        return Err(format!("Source env file not found: {}", source_path.display()));
    }

    if output_path.exists() && !opts.force {
        return Err(format!(
            "Target file '{}' already exists. Use --force to overwrite.",
            output_path.display()
        ));
    }

    let env_file = EnvFile::from_file(source_path)
        .map_err(|e| format!("Failed to read {}: {}", source_path.display(), e))?;

    let mut output_lines = Vec::new();
    let mut generated_count = 0;

    for item in &env_file.items {
        match item {
            LineItem::Empty => {
                output_lines.push(String::new());
            }
            LineItem::Comment(raw) => {
                output_lines.push(raw.clone());
            }
            LineItem::Entry(entry) => {
                generated_count += 1;
                let sanitized_val = sanitize_value(&entry.key, &entry.value, opts.keep_defaults);
                output_lines.push(format!("{}={}", entry.key, sanitized_val));
            }
        }
    }

    let output_content = output_lines.join("\n") + "\n";
    fs::write(output_path, output_content)
        .map_err(|e| format!("Failed to write to {}: {}", output_path.display(), e))?;

    Ok(generated_count)
}

fn sanitize_value(key: &str, current_val: &str, keep_defaults: bool) -> String {
    let upper_key = key.to_uppercase();
    let trimmed = current_val.trim();

    // If already empty, keep it empty
    if trimmed.is_empty() {
        return String::new();
    }

    // If keep_defaults is on, keep well-known harmless defaults
    if keep_defaults {
        let safe_keys = [
            "PORT",
            "HOST",
            "APP_PORT",
            "APP_ENV",
            "NODE_ENV",
            "DEBUG",
            "APP_DEBUG",
            "DB_PORT",
            "DB_HOST",
            "DB_CONNECTION",
            "REDIS_PORT",
            "REDIS_HOST",
        ];
        if safe_keys.contains(&upper_key.as_str()) {
            return current_val.to_string();
        }

        let safe_values = [
            "localhost",
            "127.0.0.1",
            "true",
            "false",
            "development",
            "local",
            "test",
            "utf8mb4",
        ];
        if safe_values.contains(&trimmed.to_lowercase().as_str()) {
            return current_val.to_string();
        }
    }

    // Default: format as your_<key>
    format!("your_{}_here", key.to_lowercase())
}
