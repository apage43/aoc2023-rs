use color_eyre::Result;
use std::io;
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = io::read_to_string(io::stdin())?;
    let total: u32 = input.lines().filter_map(|line| {
        let mut it = line
            .chars()
            .filter(char::is_ascii_digit)
            .filter_map(|c| c.to_digit(10));
        let first = it.next();
        let last = match it.last() {
            None => first,
            last => last,
        };
        Some(first? * 10 + last?)
    }).sum();
    println!("total: {total}");
    Ok(())
}
