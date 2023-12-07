use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

fn calc_hand_type(cards: &[Card]) -> HandType {
    let mut map: HashMap<&Card, u8> = HashMap::new();
    cards.iter().for_each(|c| {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    let mut vec_values: Vec<u8> = map.values().copied().collect();
    vec_values.sort();
    vec_values.push(0);
    vec_values.push(0);
    vec_values.push(0);
    vec_values.push(0); // pad with 4 zeroes
    vec_values = vec_values.iter().take(5).copied().collect();
    vec_values.sort();
    let values: Option<(u8, u8, u8, u8, u8)> = vec_values.iter().take(5).copied().collect_tuple();
    match values {
        Some((0, 0, 0, 0, 5)) => HandType::FiveOfKind,
        Some((0, 0, 0, 1, 4)) => HandType::FourOfKind,
        Some((0, 0, 0, 2, 3)) => HandType::FullHouse,
        Some((0, 0, 1, 1, 3)) => HandType::ThreeOfKind,
        Some((0, 0, 1, 2, 2)) => HandType::TwoPair,
        Some((_, _, _, _, 2)) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn char_to_card(c: &char) -> Option<Card> {
    match c {
        'A' => Some(Card::Ace),
        '2' => Some(Card::Two),
        '3' => Some(Card::Three),
        '4' => Some(Card::Four),
        '5' => Some(Card::Five),
        '6' => Some(Card::Six),
        '7' => Some(Card::Seven),
        '8' => Some(Card::Eight),
        '9' => Some(Card::Nine),
        'T' => Some(Card::Ten),
        'J' => Some(Card::Jack),
        'Q' => Some(Card::Queen),
        'K' => Some(Card::King),
        _ => None,
    }
}

fn char_to_card_2(c: &char) -> Option<Card> {
    match c {
        'A' => Some(Card::Ace),
        '2' => Some(Card::Two),
        '3' => Some(Card::Three),
        '4' => Some(Card::Four),
        '5' => Some(Card::Five),
        '6' => Some(Card::Six),
        '7' => Some(Card::Seven),
        '8' => Some(Card::Eight),
        '9' => Some(Card::Nine),
        'T' => Some(Card::Ten),
        'J' => Some(Card::Joker),
        'Q' => Some(Card::Queen),
        'K' => Some(Card::King),
        _ => None,
    }
}

impl Hand {
    fn new(cards: Vec<Card>, bid: u32) -> Self {
        let hand_type = calc_hand_type(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn from_str_joker(input: &str) -> Self {
        let (hand_str, bid_str) = input.split_once(' ').unwrap();
        let cards: Vec<Card> = hand_str
            .chars()
            .map(|c| char_to_card_2(&c).unwrap())
            .collect();
        let bid = bid_str.parse::<u32>().unwrap();
        let hand_type = calc_hand_type_2(&cards);
        Hand {
            cards,
            bid,
            hand_type,
        }
    }
}

fn calc_hand_type_2(cards: &[Card]) -> HandType {
    let mut primary_map: HashMap<&Card, u8> = HashMap::new();
    cards.iter().for_each(|c| {
        if c != &Card::Joker {
            primary_map.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }
    });
    primary_map.insert(&Card::Joker, 0);
    let biggest_key = primary_map
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k)
        .unwrap();
    let joker_count = cards.iter().filter(|c| c == &&Card::Joker).count() as u8;
    primary_map
        .entry(biggest_key)
        .and_modify(|x| *x += joker_count);
    let mut vec_values: Vec<u8> = primary_map.values().copied().collect();
    vec_values.sort();
    vec_values.push(0);
    vec_values.push(0);
    vec_values.push(0);
    vec_values.push(0); // pad with 4 zeroes
    vec_values = vec_values.iter().take(5).copied().collect();
    vec_values.sort();
    let values: Option<(u8, u8, u8, u8, u8)> = vec_values.iter().take(5).copied().collect_tuple();
    match values {
        Some((0, 0, 0, 0, 5)) => HandType::FiveOfKind,
        Some((0, 0, 0, 1, 4)) => HandType::FourOfKind,
        Some((0, 0, 0, 2, 3)) => HandType::FullHouse,
        Some((0, 0, 1, 1, 3)) => HandType::ThreeOfKind,
        Some((0, 0, 1, 2, 2)) => HandType::TwoPair,
        Some((_, _, _, _, 2)) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn compare_card_sets(left: &[Card], right: &[Card]) -> Ordering {
    for i in 0..left.len() {
        match left[i].cmp(&right[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }

    Ordering::Equal
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
            && compare_card_sets(&self.cards, &other.cards) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn gt(&self, other: &Self) -> bool {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => compare_card_sets(&self.cards, &other.cards) == Ordering::Greater,
            Ordering::Greater => true,
            _ => false,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => compare_card_sets(&self.cards, &other.cards) == Ordering::Less,
            Ordering::Less => true,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        self.gt(other) || self.eq(other)
    }

    fn le(&self, other: &Self) -> bool {
        self.lt(other) || self.eq(other)
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
        Self: PartialOrd,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self > other {
            self
        } else {
            other
        }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self < other {
            self
        } else {
            other
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => compare_card_sets(&self.cards, &other.cards),
            o => o,
        }
    }
}

#[derive(Debug)]
struct Day6Error;

impl FromStr for Hand {
    type Err = Day6Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((hand_str, bid)) = s.split_once(' ') {
            let cards: Vec<Card> = hand_str
                .chars()
                .map(|c| char_to_card(&c).unwrap())
                .collect();
            return match bid.parse::<u32>() {
                Ok(b) => Ok(Hand::new(cards, b)),
                Err(_) => Err(Day6Error),
            };
        }

        Err(Day6Error)
    }
}

fn hands_to_ranked_hands(hands: &mut [Hand]) -> Vec<(&Hand, u32)> {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(ix, h)| {
            let rank: u32 = ix as u32 + 1;
            (h, rank)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|l| l.parse::<Hand>().unwrap()).collect();
    let ranked = hands_to_ranked_hands(&mut hands);
    Some(ranked.iter().map(|(h, r)| h.bid * *r).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from_str_joker).collect();
    let ranked = hands_to_ranked_hands(&mut hands);
    Some(ranked.iter().map(|(h, r)| h.bid * *r).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_compare() {
        let c1 = Card::Two;
        let c2 = Card::Three;
        assert!(&c1 < &c2);
        let mut input = vec![&c1, &c2];
        input.sort();
        let expected = vec![&c1, &c2];
        assert_eq!(input, expected);
    }
}
