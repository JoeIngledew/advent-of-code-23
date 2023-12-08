use std::collections::BTreeMap;

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

struct Puzzle {
    nodes: BTreeMap<String, (String, String)>,
    instructions: Vec<Direction>,
}

impl Puzzle {
    fn steps_to_zzz(&self) -> u32 {
        let mut dir_cursor = 0;
        let mut steps: u32 = 0;
        let mut curr_node = &String::from("AAA");
        while curr_node != &String::from("ZZZ") {
            steps += 1;
            let next_dir = &self.instructions[dir_cursor];
            let choices = self.nodes.get(curr_node).unwrap();
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
}

fn parse_input(input: &str) -> Puzzle {
    let mut lines = input.lines();
    let direction_line = lines.next().unwrap();
    let instructions: Vec<Direction> = direction_line.chars().filter_map(char_to_dir).collect();
    // blank line
    lines.next();

    let mut nodes: BTreeMap<String, (String, String)> = BTreeMap::new();
    for l in lines {
        let split: Vec<String> = l
            .split(' ')
            .filter_map(|s| {
                let tmp = s.replace(|c: char| !c.is_alphabetic(), "");
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
            nodes.insert(String::from(key), (String::from(l), String::from(r)));
        }
    }

    Puzzle {
        nodes,
        instructions,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle = parse_input(input);
    Some(puzzle.steps_to_zzz())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
