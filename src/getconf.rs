use std::path::Path;
use std::fs;
use anyhow::{Result, Context};

pub fn getconf() -> Result<(Vec<(String, String, String)>)> {
    if !Path::new("/etc/raw.conf").exists() {
        println!("Raw.conf doesn't exists aborting");
        std::process::exit(1)
    }
    let rawconf = fs::read_to_string("/etc/raw.conf").context("Failed to read raw.conf, see the error below")?;
    if rawconf.contains("mode=binary") {
        let mut source = rawconf.lines().find(|l| l.starts_with("source=")).context("Failed to get source= line")?.split_once("source=").map(|(_, source)| source).context("Failed to get source path")?.split_whitespace().next().context("")?;
        let mut index = String::new();
        if source.ends_with("/") {
            source = source.trim_end_matches("/");
            index = format!("{}/index.raw", source);
        } else {
            index = format!("{}/index.raw", source);
        }
        if !Path::new(&index).exists() {
            println!("ERROR: index.raw doesn't exist, please run raw index");
            std::process::exit(1)
        }
        let index = fs::read_to_string(index)?;
        let mut localdata = Vec::new();
        for i in index.lines() {
            let i = i.split_once("_").map(|(ls, _)| ls).context("Failed to split")?;
            let path = format!("{}/{}", source, &i);
            let pkgfile = fs::read_to_string(path).context("Failed to get pkgfile")?;
            let name = pkgfile.lines().find(|l| l.contains("name=")).context("Failed to get package name")?.split_once("name=").map(|(_, name)| name).context("Failed to get pkgname")?;
            let version = pkgfile.lines().find(|l| l.contains("version=")).context("Failed to get package version")?.split_once("version=").map(|(_, name)| name).context("Failed to get pkgver")?.to_string();
            let mut release = String::new();
            if pkgfile.contains("release=") {
                release = pkgfile.lines().find(|l| l.contains("release=")).context("Failed to get package release")?.split_once("release=").map(|(_, name)| name).context("Failed to get pkgrel")?.to_string();
            } else {
                release = "1".to_string();
            }
            localdata.push((name.to_string(), version.to_string(), release.to_string()));
        }
        println!("{:?}", localdata);
        return Ok(localdata);
    } else {
        anyhow::bail!("Unsupported or missing mode in raw.conf");
    }
    //println!("{:?}", localdata);
}


