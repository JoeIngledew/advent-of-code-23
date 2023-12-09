advent_of_code::solution!(9);

use hashbrown::HashSet;

fn unique(xs: &[i64]) -> Option<i64> {
    if xs.iter().copied().collect::<HashSet<i64>>().len() == 1 {
        Some(xs[0])
    } else {
        None
    }
}

fn get_next_in_seq(seq: Vec<i64>) -> i64 {
    let mut diffs: Vec<i64> = Vec::new();
    for windows in seq.windows(2) {
        let diff = windows[1] - windows[0];
        diffs.push(diff);
    }
    match unique(&diffs) {
        Some(x) => seq.last().unwrap() + x,
        None => seq.last().unwrap() + get_next_in_seq(diffs)
    }
}

fn get_next_in_seq_back(seq: Vec<i64>) -> i64 {
    let mut diffs: Vec<i64> = Vec::new();
    for windows in seq.windows(2) {
        let diff = windows[1] - windows[0];
        diffs.push(diff);
    }
    match unique(&diffs) {
        Some(x) => seq.first().unwrap() - x,
        None => seq.first().unwrap() - get_next_in_seq_back(diffs)
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let res = input.lines().map(|l| {
        get_next_in_seq(l.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
    }).sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let res: Vec<i64> = input.lines().map(|l| {
        get_next_in_seq_back(l.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
    }).collect();
    let res = res.iter().sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
