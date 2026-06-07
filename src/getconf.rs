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
use crate::getdistant::getdistant;

pub fn getconf() -> Result<Vec<(String, String, String)> > {
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
        let mut path = String::new();
        for i in index.lines() {
            if i.matches("_").count() > 1 {
                let pos = i.rfind('_').unwrap();
                path = i[..pos].to_string();
                //path = parts.next(1).context("Failed to split")?.to_string();
            } else {
                path = i.split_once("_").map(|(ls, _)| ls).context("Failed to split")?.to_string();
            }
            //let i = i.split_once("_").map(|(ls, _)| ls).context("Failed to split")?;
            let path = format!("{}/{}", source, &path);
        
            //let path = format!("{}/{}", source, &i);
            println!("{}", path);
            let pkgfile = fs::read_to_string(&path).context("Failed to get pkgfile")?;
            let mut name = String::new();
            if !pkgfile.contains("name=") {
                name = path.split_once("/Pkgfile").map(|(name, _)| name).context("Failed to get package name")?.trim_end_matches("/").rsplit_once('/').map(|(_, name)| name).context("Failed")?.to_string();
            } else {
                let name = pkgfile.lines().find(|l| l.contains("name=")).context("Failed to get package name")?.split_once("name=").map(|(_, name)| name).context("Failed to get pkgname")?;
            }
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
        getdistant(&localdata)?;
        return Ok(localdata);
    } else {        
        if !Path::new("/etc/raw.conf").exists() {
            println!("Raw.conf doesn't exists aborting");
            std::process::exit(1)
        }
        let rawconf = fs::read_to_string("/etc/raw.conf").context("Failed to read raw.conf, see the error below")?;
        if rawconf.contains("mode=source") {
            let mut source = rawconf.lines().find(|l| l.starts_with("root=")).context("Failed to get root= line")?.split_once("root=").map(|(_, source)| source).context("Failed to get root path")?.split_whitespace().next().context("")?;
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
            let mut path = String::new();
            for i in index.lines() {
                if i.matches("_").count() > 1 {
                    let pos = i.rfind('_').unwrap();
                    path = i[..pos].to_string();
                    //path = parts.next(1).context("Failed to split")?.to_string();
                } else {
                    path = i.split_once("_").map(|(ls, _)| ls).context("Failed to split")?.to_string();
                }
                //let i = i.split_once("_").map(|(ls, _)| ls).context("Failed to split")?;
                let path = format!("{}/{}", source, &path);
                println!("{}", path);
                let pkgfile = fs::read_to_string(&path).context("Failed to get pkgfile")?;
                let mut name = String::new();
                if !pkgfile.contains("name=") {
                    name = path.split_once("/Pkgfile").map(|(name, _)| name).context("Failed to get package name")?.trim_end_matches("/").rsplit_once('/').map(|(_, name)| name).context("Failed")?.to_string();
                } else {
                    let name = pkgfile.lines().find(|l| l.contains("name=")).context("Failed to get package name")?.split_once("name=").map(|(_, name)| name).context("Failed to get pkgname")?;
                }
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
            getdistant(&localdata)?;
            return Ok(localdata);
        } else {
            anyhow::bail!("Unsupported or missing mode in raw.conf");
        }
    }
}


