use color_eyre::Result;
use std::io;
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = io::read_to_string(io::stdin())?;
    Ok(())
}
