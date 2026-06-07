use anyhow::{Result, Context};
use std::env;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn getdistant(local: &Vec<(String, String, String)>) -> Result<()> {
    let url = "https://gitlab.archlinux.org/archlinux/packaging/state.git";
    let mut opt = git2::FetchOptions::new();
    opt.depth(1);
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(opt);
    if Path::new("/var/cache/state").exists() {
        fs::remove_dir_all("/var/cache/state")?;
    }
    let _repo = match builder.clone(url, Path::new("/var/cache/state")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    }; 
    let mut unknown: Vec<String> = Vec::new();
    let mut toupdate: Vec<(String, String, String)> = Vec::new();
    env::set_current_dir("/var/cache/state/").context("Directory does not exist, cloning failed")?;
    let _pkg: Vec<_> = fs::read_dir("/var/cache/state")?
        .filter_map(|e| e.ok())
        .collect();
    let number = local.iter().count();
    println!("Package number {}", number);
    for (name, version, release) in local.iter() {
            let info = WalkDir::new("/var/cache/state")
                .into_iter()
                .filter_map(|e| e.ok())
                .find(|e| e.file_name().to_str().unwrap_or("") == name);
            if let Some(entry) = info {
                //println!("{:?}", entry);
                let data = fs::read_to_string(entry.path())?;
                //let data = data;
                let name = data.split_once(" ").map(|(name, _)| name).context("FAILED TO GET PKGNAME")?;
                let pkgver = data.split_once(name).map(|(_, ver)| ver).context("FAILED TO GET PKGVER")?.split_once(" ").map(|(_, pkgver)| pkgver).context("")?.split_once(" ").map(|(pkgver, _)| pkgver).context("FAILED TO GET PKGVER")?.split_once("-").map(|(pkgver, _)| pkgver).context("Failed to get pkgver")?;
                let pkgrel = data.split_once(pkgver).map(|(_, ver)| ver).context("FAILED TO GET PKGREL")?.split_once("-").map(|(_, pkgrel)| pkgrel).context("")?.split_once(" ").map(|(pkgrel, _)| pkgrel).context("Failed")?;
                println!("{} {} {}", name, pkgver, pkgrel);
                if version != pkgver {
                    toupdate.push((name.to_string(), pkgver.to_string(), pkgrel.to_string()));
                } else {
                    if release != pkgrel {
                        toupdate.push((name.to_string(), pkgver.to_string(), pkgrel.to_string()));
                    }
                }
            } else {
                unknown.push(name.to_string());
            }
    }
    let number = unknown.iter().count();
    println!("Unknown packages :  {}", number);
    println!("Packages to update : {:?}", toupdate);
    Ok(())

}