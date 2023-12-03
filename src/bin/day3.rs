use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Cell {
    x: usize,
    y: usize,
}

impl Cell {
    fn go(&self, xoffs: isize, yoffs: isize) -> Option<Cell> {
        let nx = (self.x as isize) + xoffs;
        let ny = (self.y as isize) + yoffs;
        if nx >= 0 && ny >= 0 {
            Some(Cell {
                x: nx as usize,
                y: ny as usize,
            })
        } else {
            None
        }
    }
    fn adjacents(&self) -> impl Iterator<Item = Cell> {
        [
            self.go(-1, -1),
            self.go(0, -1),
            self.go(1, -1),
            self.go(-1, 0),
            self.go(1, 0),
            self.go(-1, 1),
            self.go(0, 1),
            self.go(1, 1),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone)]
struct Object {
    chars: String,
    locations: Vec<Cell>,
}

impl Object {
    fn is_number(&self) -> bool {
        self.chars.chars().any(|c| c.is_ascii_digit())
    }
    fn is_symbol(&self) -> bool {
        !self.is_number()
    }
}

use color_eyre::Result;
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = std::io::read_to_string(std::io::stdin())?;
    let objects = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .filter(|(_, c)| *c != '.')
        .fold(vec![], |mut objects: Vec<Object>, ((x, y), c)| match c {
            '.' => objects,
            '0'..='9' => match objects.last_mut() {
                Some(last)
                    if last.chars.chars().last().unwrap().is_ascii_digit()
                        && last
                            .locations
                            .last()
                            .map(|loc| loc.x + 1 == x)
                            .unwrap_or(false) =>
                {
                    last.chars.push(c);
                    last.locations.push(Cell { x, y });
                    objects
                }
                _ => {
                    objects.push(Object {
                        chars: String::from(c),
                        locations: vec![Cell { x, y }],
                    });
                    objects
                }
            },
            c => {
                objects.push(Object {
                    chars: String::from(c),
                    locations: vec![Cell { x, y }],
                });
                objects
            }
        });

    let numbers: Vec<Object> = objects
        .iter()
        .filter(|&x| Object::is_number(x))
        .cloned()
        .collect();
    let symbols: Vec<_> = objects.into_iter().filter(Object::is_symbol).collect();

    let all_symbol_locations: Vec<Cell> = symbols
        .iter()
        .flat_map(|obj| obj.locations.iter().copied())
        .collect();

    let part_numbers: Vec<u32> = numbers
        .iter()
        .filter(|object| {
            let all_adjacents: HashSet<Cell> =
                HashSet::from_iter(object.locations.iter().flat_map(Cell::adjacents));
            let keep = all_symbol_locations
                .iter()
                .any(|loc| all_adjacents.contains(loc));
            keep
        })
        .map(|object| -> Result<u32> { Ok(object.chars.parse::<u32>()?) })
        .collect::<Result<_>>()?;
    let pnsum: u32 = part_numbers.iter().sum();
    dbg!(pnsum);

    let gear_groups: HashMap<usize, Vec<u32>> =
        numbers.iter().fold(HashMap::new(), |mut gmap, pn| {
            let all_adjacents: HashSet<Cell> =
                HashSet::from_iter(pn.locations.iter().flat_map(Cell::adjacents));
            let adjacent_sybols: Vec<usize> = symbols
                .iter()
                .enumerate()
                .filter_map(|(i, obj)| {
                    obj.locations
                        .iter()
                        .any(|loc| all_adjacents.contains(loc))
                        .then_some(i)
                })
                .collect();
            for s in adjacent_sybols {
                gmap.entry(s)
                    .or_default()
                    .push(pn.chars.parse::<u32>().unwrap());
            }
            gmap
        });
    let gratio: u32 = gear_groups
        .values()
        .filter(|gg| gg.len() == 2)
        .map(|gg| gg.iter().product::<u32>())
        .sum();
    dbg!(gratio);
    Ok(())
}
