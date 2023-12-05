use color_eyre::{
    eyre::{bail, eyre},
    Result,
};

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    // dst, src, len
    maps: Vec<(usize, usize, usize)>,
}
#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Mapping>,
}
fn read_almanac() -> Result<Almanac> {
    let mut seeds = None;
    let mut maps = vec![];
    for line in std::io::stdin().lines() {
        let line = line?;
        if line.starts_with("seeds: ") {
            let (_, snums) = line.split_once(':').unwrap();
            seeds = Some(
                snums
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().map_err(|e| eyre!("{e}")))
                    .collect::<Result<Vec<_>>>()?,
            );
        }
        if line.ends_with("map:") {
            let which: Vec<&str> = line.split_whitespace().next().unwrap().split('-').collect();
            maps.push(Mapping {
                from: which[0].to_owned(),
                to: which[2].to_owned(),
                maps: vec![],
            });
        }
        if line.len() > 0 && line.chars().next().unwrap().is_ascii_digit() {
            let mut parts = line.split_ascii_whitespace().map(|s| s.parse::<usize>());
            let dst = parts.next().unwrap()?;
            let src = parts.next().unwrap()?;
            let len = parts.next().unwrap()?;
            maps.last_mut().unwrap().maps.push((dst, src, len));
        }
    }
    let seeds = if let Some(seeds) = seeds {
        seeds
    } else {
        bail!("seeds")
    };
    Ok(Almanac { seeds, maps })
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let almanac = read_almanac()?;
    dbg!(almanac);
    Ok(())
}
