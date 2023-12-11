use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum NodeType {
    Space,
    Galaxy,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct ParseNodeTypeError;

impl TryFrom<char> for NodeType {
    type Error = ParseNodeTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(NodeType::Space),
            '#' => Ok(NodeType::Galaxy),
            _ => Err(ParseNodeTypeError),
        }
    }
}

fn parse_init_map(input: &str) -> (HashMap<Point, NodeType>, usize, usize) {
    let mut map: HashMap<Point, NodeType> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Ok(t) = NodeType::try_from(c) {
                map.insert_unique_unchecked(Point { x, y }, t);
            }
            max_x = max_x.max(x);
        }
        max_y = max_y.max(y);
    }
    (map, max_x, max_y)
}

struct ColsRows {
    xs: Vec<usize>,
    ys: Vec<usize>,
}

fn get_expanded_cols_rows(map: &HashMap<Point, NodeType>, x_len: usize, y_len: usize) -> ColsRows {
    let mut xs: Vec<usize> = Vec::new();
    for x in 0..x_len {
        if (0..y_len).all(|y| {
            map.get(&Point { x, y })
                .is_some_and(|v| v == &NodeType::Space)
        }) {
            xs.push(x);
        }
    }

    let mut ys: Vec<usize> = Vec::new();
    for y in 0..y_len {
        if (0..x_len).all(|x| {
            map.get(&Point { x, y })
                .is_some_and(|v| v == &NodeType::Space)
        }) {
            ys.push(y);
        }
    }
    ColsRows { xs, ys }
}

fn get_effective_point(p: &Point, expansions: &ColsRows, expand_times: usize) -> Point {
    let effective_x = p.x + (expand_times * expansions.xs.iter().filter(|x| **x < p.x).count());
    let effective_y = p.y + (expand_times * expansions.ys.iter().filter(|y| **y < p.y).count());
    Point {
        x: effective_x,
        y: effective_y,
    }
}

fn calc_distance(p1: &Point, p2: &Point, expansions: &ColsRows, expand_times: usize) -> usize {
    let effective_p1 = get_effective_point(p1, expansions, expand_times);
    let effective_p2 = get_effective_point(p2, expansions, expand_times);
    let raw_distance_x = effective_p1.x.abs_diff(effective_p2.x);
    let raw_distance_y = effective_p1.y.abs_diff(effective_p2.y);
    raw_distance_x + raw_distance_y
}

fn get_distances(
    map: &HashMap<Point, NodeType>,
    expansions: &ColsRows,
    expand_times: usize,
) -> Vec<usize> {
    let galaxies = map
        .iter()
        .filter_map(|(k, v)| match v {
            NodeType::Galaxy => Some(k),
            _ => None,
        })
        .tuple_combinations()
        .map(|(p1, p2)| calc_distance(p1, p2, expansions, expand_times))
        .collect_vec();
    galaxies
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, max_x, max_y) = parse_init_map(input);
    let expansions = get_expanded_cols_rows(&map, max_x + 1, max_y + 1);
    let distances = get_distances(&map, &expansions, 1);
    Some(distances.iter().sum())
    //None
}

fn part_two_inner(input: &str, expand_times: usize) -> Option<usize> {
    let (map, max_x, max_y) = parse_init_map(input);
    let expansions = get_expanded_cols_rows(&map, max_x + 1, max_y + 1);
    let distances = get_distances(&map, &expansions, expand_times);
    Some(distances.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_inner(input, 1000000 - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_10() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 9);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_100() {
        let result = part_two_inner(&advent_of_code::template::read_file("examples", DAY), 99);
        assert_eq!(result, Some(8410));
    }
}
