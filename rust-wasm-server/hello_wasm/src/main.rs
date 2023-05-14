use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    let content = fs::read_to_string("./text.txt")?;
    println!("{}", content);

    Ok(())
}
