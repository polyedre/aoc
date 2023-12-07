use std::{io,fmt,cmp};
use itertools::Itertools;
use std::collections::HashMap;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, EnumString)]
enum Card {
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "6")]
    Six,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "8")]
    Eight,
    #[strum(serialize = "9")]
    Nine,
    #[strum(serialize = "T")]
    Ten,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    Knight,
    #[strum(serialize = "A")]
    As,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: String,
    category: HandType,
    bid: u32
}

impl Hand {
    fn new(cards: String, bid: u32) -> Self {
        let sorted_cards: String = cards.chars().sorted().rev().collect();

        let mut counts_by_card: HashMap<char, u32> = HashMap::new();

        for char in sorted_cards.chars() {
            match counts_by_card.get_mut(&char) {
                Some(element) => { *element += 1; }
                None => { counts_by_card.insert(char, 1); }
            }
        }

        let counts: Vec<u32> = counts_by_card.into_values().sorted().rev().collect();

        let category = match counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPairs,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Vec {:?} has not been matched", counts)
        };

        Hand { cards, category, bid }
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}(\"{}\", {})", self.category, self.cards, self.bid)
    }
}

impl cmp::PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.category != other.category { return false; }
        return self.cards == other.cards;
    }

    fn ne(&self, other: &Self) -> bool {
        !Self::eq(&self, other)
    }
}

impl cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.category != other.category { return HandType::partial_cmp(&self.category, &other.category) }
        return Vec::partial_cmp(
            &self.cards.chars().map(|c| c.to_string().parse::<Card>().unwrap()).collect::<Vec<Card>>(),
            &other.cards.chars().map(|c| c.to_string().parse::<Card>().unwrap()).collect::<Vec<Card>>(),
        )
    }
}

impl cmp::Eq for Hand {
}

impl cmp::Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        Hand::partial_cmp(&self, other).unwrap()
    }}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();

    for line in io::stdin().lines() {
        let line = line.unwrap();

        let [cards, bid]: [&str; 2] = line.split_whitespace().collect::<Vec<&str>>().try_into().unwrap();

        hands.push(Hand::new(cards.to_string(), bid.parse().unwrap()))
    }

    hands.sort();

    let solution = hands.iter().enumerate().fold(0, |previous, (index, card)| previous + card.bid * (index as u32 + 1));
    println!("Solution part 1: {}", solution);
}
