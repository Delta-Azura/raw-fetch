use git2::Repository;
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
    let repo = match builder.clone(url, Path::new("/var/cache/state")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    }; 
    let mut unknown = Vec::new();
    env::set_current_dir("/var/cache/state/").context("Directory does not exist, cloning failed")?;
    let pkg: Vec<_> = fs::read_dir("/var/cache/state")?
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
                println!("{:?}", entry);
            } else {
                unknown.push(name);
            }
    }
    let number = unknown.iter().count();
    println!("Unknown packages :  {}", number);
    Ok(())

}