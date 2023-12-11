// NOTE: Not my own work - this is copied from the github link below in an attempt to understand how to solve this problem.
// I do not think this attempt was successful, I still do not properly understand it.

// link: https://github.com/agausmann/puzzles/blob/52b3d7ca715d8b51d9ba3d9a595a496a449d10e9/adventofcode/aoc2023/src/bin/day10.rs

use std::{cmp::Reverse, collections::BinaryHeap};

use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct MapTile {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    start: bool,
}

impl From<char> for MapTile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self {
                north: true,
                south: true,
                ..Default::default()
            },
            '-' => Self {
                east: true,
                west: true,
                ..Default::default()
            },
            'L' => Self {
                north: true,
                east: true,
                ..Default::default()
            },
            'J' => Self {
                north: true,
                west: true,
                ..Default::default()
            },
            'F' => Self {
                south: true,
                east: true,
                ..Default::default()
            },
            '7' => Self {
                south: true,
                west: true,
                ..Default::default()
            },
            'S' => Self {
                start: true,
                ..Default::default()
            },
            _ => Default::default(),
        }
    }
}

fn get_map(input: &str) -> (Vec<Vec<MapTile>>, (usize, usize)) {
    let mut start: Option<(usize, usize)> = None;
    let mut map = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        // horrid mutation of a variable in a map, just to avoid having to find the start later
                        // this has to be y,x because of the way I'm constructing the grid (ie. sideways???)
                        start = Some((y, x));
                    }
                    MapTile::from(c)
                })
                .collect_vec()
        })
        .collect_vec();

    let start = start.unwrap();
    map[start.0][start.1].north = start.0 > 0 && map[start.0 - 1][start.1].south;
    map[start.0][start.1].south = start.0 < map.len() - 1 && map[start.0 + 1][start.1].north;
    map[start.0][start.1].west = start.1 > 0 && map[start.0][start.1 - 1].east;
    map[start.0][start.1].east = start.1 < map[start.0].len() - 1 && map[start.0][start.1 + 1].west;

    (map, start)
}

fn get_far_point_distance(map: &[Vec<MapTile>], start: (usize, usize)) -> usize {
    let mut dist = 0;
    let mut heap = BinaryHeap::new();
    let mut visits: HashSet<(usize, usize)> = HashSet::new();
    heap.push((Reverse(0), start));

    while let Some((Reverse(k), (i, j))) = heap.pop() {
        if !visits.insert((i, j)) {
            continue;
        }
        dist = dist.max(k);
        let c = map[i][j];
        if c.north {
            heap.push((Reverse(k + 1), (i - 1, j)));
        }
        if c.south {
            heap.push((Reverse(k + 1), (i + 1, j)));
        }
        if c.west {
            heap.push((Reverse(k + 1), (i, j - 1)));
        }
        if c.east {
            heap.push((Reverse(k + 1), (i, j + 1)));
        }
    }

    dist
}

// way too slow. todo!()
pub fn part_one(input: &str) -> Option<usize> {
    let (map, start) = get_map(input);
    let res = get_far_point_distance(&map, start);
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
