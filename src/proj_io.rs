use std::fs::File;
use std::io::prelude::*;

pub fn write_out(text: &str, path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(text.as_bytes())?;
    Ok(())
}
