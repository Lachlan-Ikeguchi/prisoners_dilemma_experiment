use crate::models::Contestant;
use std::fs;
use mlua::Result;

/// Load all contestants from a directory
pub fn load_contestants(dir: &str) -> Result<Vec<Contestant>> {
    let paths = fs::read_dir(dir)?;
    let mut contestants = Vec::new();

    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("lua") {
            let content = fs::read_to_string(&path)?;
            let name = extract_name(&content);
            let description = extract_description(&content);
            let script_path = path.to_string_lossy().into_owned();

            contestants.push(Contestant::new(name, description, script_path));
        }
    }

    Ok(contestants)
}

/// Extract the name from Lua script comments
pub fn extract_name(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-- name:") {
            return trimmed.trim_start_matches("-- name:").trim().to_string();
        }
    }
    "Unknown".to_string()
}

/// Extract the description from Lua script comments
pub fn extract_description(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("-- will")
            || (trimmed.starts_with("-- ") && !trimmed.starts_with("-- name:"))
        {
            return trimmed.trim_start_matches("--").trim().to_string();
        }
    }
    "No description".to_string()
}