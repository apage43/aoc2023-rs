use color_eyre::{eyre::bail, Result};
use scan_fmt::scan_fmt;
use std::{collections::HashMap, io, str::FromStr};

#[derive(Copy, Clone, Eq, Debug, Hash, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        use Color::*;
        Ok(match s {
            "red" => Red,
            "blue" => Blue,
            "green" => Green,
            unk => bail!("color name: {unk}"),
        })
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct ColorQuantity(Color, u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]

struct Game {
    game_id: u32,
    turns: Vec<Vec<ColorQuantity>>,
}

#[derive(Debug)]
struct Bag {
    contents: HashMap<Color, u32>,
}

impl Default for Bag {
    fn default() -> Self {
        Self {
            contents: HashMap::from_iter([(Color::Red, 0), (Color::Green, 0), (Color::Blue, 0)]),
        }
    }
}

fn game_is_possible(bag: &Bag, game: &Game) -> bool {
    for turn in game.turns.iter() {
        for cq in turn {
            if bag.contents.get(&cq.0).expect("missing color") < &cq.1 {
                return false;
            }
        }
    }
    true
}

impl Bag {
    fn power(&self) -> u32 {
        self.contents.values().product()
    }
}

fn minimum_bag(game: &Game) -> Bag {
    let mut bag = Bag::default();
    for turn in game.turns.iter() {
        for cq in turn {
            let slot = bag.contents.get_mut(&cq.0).expect("missing color");
            if *slot < cq.1 {
                *slot = cq.1;
            }
        }
    }
    bag
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = io::read_to_string(io::stdin())?;
    let games: Vec<_> = input
        .lines()
        .map(|line| -> Result<Game> {
            let number = scan_fmt!(line, "Game {}:", u32)?;
            let rest = line.split(':').last().unwrap();
            let turns: Vec<_> = rest
                .split(';')
                .map(|turn| -> Vec<_> {
                    turn.split(',')
                        .flat_map(|quantstr| -> Result<_> {
                            let mut parts = quantstr.trim().split_ascii_whitespace();
                            let quant = parts.next().expect("quantity");
                            let color = parts.next().expect("color");
                            Ok(ColorQuantity(
                                Color::from_str(color)?,
                                u32::from_str(quant)?,
                            ))
                        })
                        .collect()
                })
                .collect();
            Ok(Game {
                game_id: number,
                turns,
            })
        })
        .collect::<Result<_>>()?;
    let p1bag = Bag {
        contents: HashMap::from_iter([(Color::Red, 12), (Color::Blue, 14), (Color::Green, 13)]),
    };
    let possible_num_total: u32 = games
        .iter()
        .filter_map(|game| game_is_possible(&p1bag, game).then_some(game.game_id))
        .sum();
    eprintln!("part1");
    dbg!(possible_num_total);
    eprintln!("part2");
    let min_power_total: u32 = games.iter().map(minimum_bag).map(|bag| bag.power()).sum();
    dbg!(min_power_total);
    Ok(())
}
