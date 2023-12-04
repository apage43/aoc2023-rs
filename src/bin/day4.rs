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
        self.have.iter().filter_map(|h| self.winners.contains(h).then_some(1)).sum()
    }
    fn points(&self) -> usize {
       (0..self.num_winners()).fold(0, |v, _| if v == 0  {1 } else { v * 2})
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
        eprintln!("card {}, {} winners, {} points", card.id, card.num_winners(), card.points());
    }
    //dbg!(&cards);
    let points: usize = cards.iter().map(Card::points).sum();
    dbg!(points);
    Ok(())
}
