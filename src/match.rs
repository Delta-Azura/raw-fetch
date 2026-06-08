use std::path::Path;
use std::fs;
use anyhow::{Result, Context};

pub fn compare(name: &str) -> Result<(String, bool)> {
    if Path::new("/etc/raw-fetch").exists() {
        let fetch = fs::read_to_string("/etc/raw-fetch").context("Unable to read /etc/raw-fetch")?;
        if let Some(line) = fetch.lines().find(|l| l.starts_with(&format!("{}=", name))) {
            let pattern = line.split_once("=").map(|(_, pattern)| pattern).context("Failed parsing")?;
            return Ok((pattern.to_string(), true));
        } else {
            return Ok(("none".to_string(), false));
        };
    } else {
        return Ok(("none".to_string(), false));
    }
}