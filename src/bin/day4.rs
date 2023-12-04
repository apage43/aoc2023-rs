use std::collections::HashMap;

use color_eyre::Result;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Card {
    id: usize,
    winners: Vec<u32>,
    have: Vec<u32>,
}

impl Card {
    fn num_winners(&self) -> usize {
        self.have
            .iter()
            .filter_map(|h| self.winners.contains(h).then_some(1))
            .sum()
    }
    fn points(&self) -> usize {
        (0..self.num_winners()).fold(0, |v, _| if v == 0 { 1 } else { v * 2 })
    }
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let input = std::io::read_to_string(std::io::stdin())?;
    let cards = input
        .lines()
        .map(|line| {
            let (numpart, cardpart) = line.split_once(':').expect(":");
            let (winpart, havepart) = cardpart.split_once('|').expect("|");
            Card {
                id: scan_fmt!(numpart, "Card {}", usize).expect("card num"),
                winners: winpart
                    .split_whitespace()
                    .map(|s| s.trim().parse().expect("parse num"))
                    .collect(),
                have: havepart
                    .split_whitespace()
                    .map(|s| s.trim().parse().expect("parse num"))
                    .collect(),
            }
        })
        .collect::<Vec<_>>();
    for card in &cards {
        eprintln!(
            "card {}, {} winners, {} points",
            card.id,
            card.num_winners(),
            card.points()
        );
    }
    let points: usize = cards.iter().map(Card::points).sum();
    dbg!(points);

    let cards_by_id: HashMap<usize, Card> =
        HashMap::from_iter(cards.into_iter().map(|c| (c.id, c)));
    let mut queued_cards: Vec<usize> = cards_by_id.keys().copied().collect();
    let mut processed_cards: HashMap<usize, usize> = Default::default();
    loop {
        let mut won_cards = vec![];
        for card in queued_cards.iter() {
            let won = cards_by_id[card].num_winners();
            won_cards.extend(
                ((card + 1)..)
                    .take(won)
                    .take_while(|c| cards_by_id.contains_key(c)),
            );
            *processed_cards.entry(*card).or_default() += 1;
        }
        if won_cards.is_empty() { break }
        queued_cards.clear();
        queued_cards.append(&mut won_cards);
    }
    dbg!(&processed_cards);
    dbg!(processed_cards.values().sum::<usize>());

    Ok(())
}
