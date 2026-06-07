mod getconf;
mod getdistant;
use getconf::getconf;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    getconf()?;
    Ok(())
}