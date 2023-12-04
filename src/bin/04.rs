advent_of_code::solution!(4);

use std::collections::HashMap;

use array_tool::vec::Intersect;

struct Card {
    id: usize,
    winning_nums: Vec<u32>,
    card_nums: Vec<u32>,
}

fn double(x: u32, i: usize) -> u32 {
    match (i, x) {
        (0, x) => x,
        (i_n, 0) => double(1, i_n - 1),
        (i_n, x_n) => double(x_n * 2, i_n - 1),
    }
}

impl Card {
    fn win_count(&self) -> usize {
        self.winning_nums.intersect(self.card_nums.clone()).len()
    }

    fn calc_score(&self) -> u32 {
        let match_count = self.win_count();
        double(0, match_count)
    }
}

fn parse_line(line: &str) -> Card {
    let (card_id, nums) = line.split_once(':').unwrap();
    let (win, rest) = nums.split_once('|').unwrap();
    let winning_nums: Vec<u32> = win
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let card_nums: Vec<u32> = rest
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();
    let id = card_id
        .split_once(' ')
        .unwrap()
        .1
        .trim()
        .parse::<usize>()
        .unwrap();
    Card {
        id,
        winning_nums,
        card_nums,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|l| parse_line(l).calc_score()).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<Card> = input.lines().map(parse_line).collect();
    let mut card_counts = HashMap::<usize, u32>::new();
    for card in &cards {
        card_counts.insert(card.id, 1);
    }
    for card in cards {
        let score = card.win_count();
        let existing_count = match card_counts.get(&card.id) {
            Some(n) => *n,
            None => 1,
        };
        for i in 1..=score {
            let id = card.id + i;
            card_counts.entry(id).and_modify(|v| *v += existing_count);
        }
    }
    Some(card_counts.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
