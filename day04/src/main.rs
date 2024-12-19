use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};

struct Card {
    winning: HashSet<u32>,
    drawn: Vec<u32>,
}

fn main() {
    let cards = read_input(); 

    let (score, num_cards) = get_score(&cards);

    println!("Score of winning cards: {}", score);
    println!("Number of scratch cards: {}", num_cards);
}

fn read_input() -> Vec<Card> {
    let mut s = String::new();
    let _ = File::open("input04.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let mut cards: Vec<Card> = Vec::new();

    s
        .trim()
        .lines()
        .for_each(|line| {
            let (_, draws) = line.split_once(":").expect("Could not split at :");

            let mut card = Card {
                winning: HashSet::new(),
                drawn: Vec::new(),
            };

            let (winnings, draws) = draws.split_once("|").expect("Could not split winnings from drawn");
            card.winning = winnings
                .split_whitespace()
                .map(|c| c.parse::<u32>().expect("Could not parse winning card."))
                .collect::<HashSet<u32>>();
            card.drawn = draws
                .split_whitespace()
                .map(|c| c.parse::<u32>().expect("Could not parse drawn card."))
                .collect::<Vec<u32>>();

            cards.push(card);
        });

    cards
}

fn get_score(cards: &Vec<Card>) -> (i64, i64) {
    let mut score: i64 = 0;
    let mut num: i64 = 0;
    let mut copies: HashMap<usize, i64> = HashMap::new();
    for (i, card) in cards.iter().enumerate() {
        let num_copies = if copies.contains_key(&i) { *copies.get(&i).unwrap() } else { 1 };
        let num_wins = card.drawn
            .iter()
            .filter(|c| card.winning.contains(&c))
            .count();

        for j in 1..num_wins+1 {
            let idx = j + i;
            if idx < cards.len() { 
                copies.entry(idx)
                    .and_modify(|v| *v += num_copies)
                    .or_insert(num_copies + 1);
            }
        }

        if num_wins > 0 { score += 1 << (num_wins - 1); }
        num += num_copies;
    }
    (score, num)
}
