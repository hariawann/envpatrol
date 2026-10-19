use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct EnvEntry {
    pub key: String,
    pub value: String,
    pub line_number: usize,
    pub has_export: bool,
    pub inline_comment: Option<String>,
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

            // Support "export KEY=VAL"
            let (has_export, effective_line) = if let Some(stripped) = trimmed.strip_prefix("export ") {
                (true, stripped.trim_start())
            } else {
                (false, trimmed)
            };

            if let Some((key_part, val_part)) = effective_line.split_once('=') {
                let key = key_part.trim().to_string();

                if key.is_empty() {
                    items.push(LineItem::Comment(line.to_string()));
                    continue;
                }

                let (val, inline_comment) = parse_value_and_comment(val_part);

                let entry = EnvEntry {
                    key: key.clone(),
                    value: val,
                    line_number,
                    has_export,
                    inline_comment,
                    raw: line.to_string(),
                };

                entries.insert(key, entry.clone());
                items.push(LineItem::Entry(entry));
            } else {
                // Malformed line preserved as comment
                items.push(LineItem::Comment(line.to_string()));
            }
        }

        Self { entries, items }
    }
}

fn parse_value_and_comment(raw_val: &str) -> (String, Option<String>) {
    let trimmed = raw_val.trim();
    if trimmed.is_empty() {
        return (String::new(), None);
    }

    // Handle double-quoted string: "value" # optional comment
    if trimmed.starts_with('"') {
        if let Some(end_quote_idx) = trimmed[1..].find('"') {
            let val = &trimmed[1..=end_quote_idx];
            let remainder = trimmed[end_quote_idx + 2..].trim();
            let comment = if let Some(stripped) = remainder.strip_prefix('#') {
                Some(stripped.trim().to_string())
            } else {
                None
            };
            return (val.to_string(), comment);
        }
    }

    // Handle single-quoted string: 'value' # optional comment
    if trimmed.starts_with('\'') {
        if let Some(end_quote_idx) = trimmed[1..].find('\'') {
            let val = &trimmed[1..=end_quote_idx];
            let remainder = trimmed[end_quote_idx + 2..].trim();
            let comment = if let Some(stripped) = remainder.strip_prefix('#') {
                Some(stripped.trim().to_string())
            } else {
                None
            };
            return (val.to_string(), comment);
        }
    }

    // Unquoted value with potential inline comment (e.g. PORT=3000 # app port)
    if let Some((val_part, comment_part)) = trimmed.split_once(" #") {
        return (
            val_part.trim().to_string(),
            Some(comment_part.trim().to_string()),
        );
    }

    (trimmed.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_env() {
        let content = r#"
# Comment
PORT=3000 # App Port
DB_HOST="127.0.0.1" # Database
export API_KEY='secret123'
EMPTY_VAR=
"#;
        let env_file = EnvFile::parse(content);
        assert_eq!(env_file.entries.len(), 4);
        assert_eq!(env_file.entries.get("PORT").unwrap().value, "3000");
        assert_eq!(
            env_file.entries.get("PORT").unwrap().inline_comment.as_deref(),
            Some("App Port")
        );
        assert_eq!(env_file.entries.get("DB_HOST").unwrap().value, "127.0.0.1");
        assert_eq!(env_file.entries.get("API_KEY").unwrap().value, "secret123");
        assert!(env_file.entries.get("API_KEY").unwrap().has_export);
        assert_eq!(env_file.entries.get("EMPTY_VAR").unwrap().value, "");
    }
}
