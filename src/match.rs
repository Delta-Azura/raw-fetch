// Raw-fetch is a simple fetcher that looks at archlinux version of packages to know if yours are up to date or not.
//    Copyright (C) 2026  Alexis/Delta-Azura

//    This program is free software; you can redistribute it and/or modify
//    it under the terms of the GNU General Public License as published by
//    the Free Software Foundation; either version 2 of the License, or
//    (at your option) any later version.

//    This program is distributed in the hope that it will be useful,
//    but WITHOUT ANY WARRANTY; without even the implied warranty of
//    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//    GNU General Public License for more details.

//    You should have received a copy of the GNU General Public License along
//    with this program; if not, write to the Free Software Foundation, Inc.,
//    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.


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