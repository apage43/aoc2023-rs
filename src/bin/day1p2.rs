use color_eyre::Result;
use std::io;
fn main() -> Result<()> {
    color_eyre::install()?;
    let named_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let input = io::read_to_string(io::stdin())?;
    let total: u32 = input
        .lines()
        .filter_map(|line| {
            let mut it = (0..line.len()).filter_map(|idx| {
                let (_, rest) = line.split_at(idx);
                if let Some(ch) = rest.chars().next() {
                    if ch.is_ascii_digit() {
                        return ch.to_digit(10);
                    }
                }
                for (d, name) in named_digits.iter().enumerate() {
                    if rest.starts_with(name) {
                        return Some(d as u32 + 1);
                    }
                }
                None
            });

            let first = it.next();
            let last = match it.last() {
                None => first,
                last => last,
            };
            Some(first? * 10 + last?)
        })
        .sum();
    println!("total: {total}");
    Ok(())
}
