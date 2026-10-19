use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct EnvEntry {
    pub key: String,
    pub value: String,
    pub line_number: usize,
    #[allow(dead_code)]
    pub raw: String,
}

#[derive(Debug, Clone)]
pub enum LineItem {
    Comment(String),
    Empty,
    Entry(EnvEntry),
}

#[derive(Debug, Default)]
pub struct EnvFile {
    pub entries: BTreeMap<String, EnvEntry>,
    pub items: Vec<LineItem>,
}

impl EnvFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(Self::parse(&content))
    }

    pub fn parse(content: &str) -> Self {
        let mut entries = BTreeMap::new();
        let mut items = Vec::new();

        for (idx, line) in content.lines().enumerate() {
            let line_number = idx + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                items.push(LineItem::Empty);
                continue;
            }

            if trimmed.starts_with('#') {
                items.push(LineItem::Comment(line.to_string()));
                continue;
            }

            // Support "export KEY=VAL" as well as "KEY=VAL"
            let effective_line = if let Some(stripped) = trimmed.strip_prefix("export ") {
                stripped.trim_start()
            } else {
                trimmed
            };

            if let Some((key_part, val_part)) = effective_line.split_once('=') {
                let key = key_part.trim().to_string();
                let mut val = val_part.trim().to_string();

                // Strip surrounding single or double quotes
                if (val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\''))
                {
                    if val.len() >= 2 {
                        val = val[1..val.len() - 1].to_string();
                    }
                }

                let entry = EnvEntry {
                    key: key.clone(),
                    value: val,
                    line_number,
                    raw: line.to_string(),
                };

                entries.insert(key, entry.clone());
                items.push(LineItem::Entry(entry));
            } else {
                // Unknown/malformed line treated as raw comment or preserved
                items.push(LineItem::Comment(line.to_string()));
            }
        }

        Self { entries, items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_env() {
        let content = r#"
# Comment
PORT=3000
DB_HOST="127.0.0.1"
export API_KEY='secret123'
"#;
        let env_file = EnvFile::parse(content);
        assert_eq!(env_file.entries.len(), 3);
        assert_eq!(env_file.entries.get("PORT").unwrap().value, "3000");
        assert_eq!(env_file.entries.get("DB_HOST").unwrap().value, "127.0.0.1");
        assert_eq!(env_file.entries.get("API_KEY").unwrap().value, "secret123");
    }
}
