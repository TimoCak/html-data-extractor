use std::io::Result;
use std::fs;

pub mod website;

pub fn create_initial_dir() -> Result<()> {
    fs::create_dir_all("output/")?;
    Ok(())
}