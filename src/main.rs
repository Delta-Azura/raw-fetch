mod getconf;
use getconf::getconf;
use anyhow::{Result, Context};

fn main() -> Result<()> {
    println!("Hello, world!");
    getconf()?;
    Ok(())
}