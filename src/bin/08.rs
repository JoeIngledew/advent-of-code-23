use std::{collections::{HashMap, BTreeMap}, hash::Hash};
use rayon::prelude::*;

advent_of_code::solution!(8);

enum Direction {
    Left,
    Right,
}

fn char_to_dir(c: char) -> Option<Direction> {
    match c {
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Right),
        _ => None,
    }
}

#[derive(PartialOrd, Ord)]
struct NodeKey {
    key: String,
    is_start: bool,
    is_end: bool
}

impl PartialEq for NodeKey {
    fn eq(&self, other: &Self) -> bool {
        &self.key == &other.key
    }
}

impl Hash for NodeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }

    fn hash_slice<H: std::hash::Hasher>(data: &[Self], state: &mut H)
        where
            Self: Sized, {
        data.hash(state);
    }
}

impl Eq for NodeKey {}

struct Puzzle {
    nodes: HashMap<NodeKey, (String, String)>,
    instructions: Vec<Direction>,
}

impl Puzzle {
    fn steps_to_zzz(&self) -> u64 {
        let mut dir_cursor = 0;
        let mut steps: u64 = 0;
        let mut curr_node = &String::from("AAA");
        while curr_node != &String::from("ZZZ") {
            steps += 1;
            let next_dir = &self.instructions[dir_cursor];
            let choices = self.nodes.get(&NodeKey { key: curr_node.to_string(), is_start: false, is_end: false }).unwrap();
            match next_dir {
                Direction::Left => curr_node = &choices.0,
                Direction::Right => curr_node = &choices.1,
            }
            dir_cursor += 1;
            if dir_cursor == self.instructions.len() {
                dir_cursor = 0;
            }
        }
        steps
    }

    fn steps_to_z(&self) -> usize {
        let start_keys: Vec<&NodeKey> = self.nodes.keys().filter(|x| x.is_start).collect();
        let mut key_index: BTreeMap<String, &NodeKey> = BTreeMap::new();
        for x in &self.nodes {
            let k = x.0;
            key_index.insert(k.key.to_string(), k);
        }
        let steps_iter: Vec<usize> = start_keys.par_iter().map(|k| {
            let mut dir_cursor = 0;
            let mut steps: u64 = 0;
            let mut curr_node = k;
            while !curr_node.is_end {
                steps += 1;
                let next_dir = &self.instructions[dir_cursor];
                let choices = self.nodes.get(curr_node).unwrap();
                match next_dir {
                    Direction::Left => curr_node = key_index.get(&choices.0).unwrap(),
                    Direction::Right => curr_node = key_index.get(&choices.1).unwrap(),
                }
                dir_cursor += 1;
                if dir_cursor == self.instructions.len() {
                    dir_cursor = 0;
                }
            }
            steps as usize
        }).collect();
        lcm(&steps_iter)
    }
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn parse_input(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let direction_line = lines.next().unwrap();
    let instructions: Vec<Direction> = direction_line.chars().filter_map(char_to_dir).collect();
    // blank line
    lines.next();

    let mut nodes: HashMap<NodeKey, (String, String)> = HashMap::new();
    for l in lines {
        let split: Vec<String> = l
            .split(' ')
            .filter_map(|s| {
                let tmp = s.replace(|c: char| !c.is_alphanumeric(), "");
                match tmp.is_empty() {
                    true => None,
                    false => Some(tmp),
                }
            })
            .collect();
        if split.len() == 3 {
            let key = split[0].as_str();
            let l = split[1].as_str();
            let r = split[2].as_str();
            let is_start = key.ends_with('A');
            let is_end = key.ends_with('Z');
            nodes.insert(NodeKey { key: String::from(key), is_start, is_end }, (String::from(l), String::from(r)));
        }
    }

    Puzzle {
        nodes,
        instructions,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = parse_input(input);
    Some(puzzle.steps_to_zzz())
}

pub fn part_two(input: &str) -> Option<usize> {
    let puzzle = parse_input(input);
    Some(puzzle.steps_to_z())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn compare_nodes() {
        let a = NodeKey { is_end: false, is_start: true, key: String::from("AAA") };
        let b_key: &String = &"AAA".to_string();
        let b = NodeKey { is_end: true, is_start: false, key: b_key.to_string() };
        assert!(a.eq(&b));
    }

    #[test]
    fn map_test() {
        let a = NodeKey { is_end: false, is_start: true, key: String::from("AAA") };
        let b_key: &String = &"AAA".to_string();
        let b = NodeKey { is_end: true, is_start: false, key: b_key.to_string() };
        let mut map: HashMap<NodeKey, u8> = HashMap::new();
        map.insert(a, 1);
        let entry = map.get(&b);
        assert!(entry.is_some());
    }
}
