// use hashbrown::{HashMap, HashSet};
// use itertools::Itertools;

advent_of_code::solution!(10);

// #[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
// struct Point {
//     x: usize,
//     y: usize
// }

// #[derive(Debug)]
// struct MapCell {
//     coords: Point,
//     east: bool,
//     west: bool,
//     north: bool,
//     south: bool,
//     start: bool
// }

// impl MapCell {
//     fn from_map_char(x: usize, y: usize, char: char) -> Option<Self> {
//         let p = Point { x, y };
//         match char {
//             '|' => Some(MapCell { coords: p, east: false, west: false, north: true, south: true, start: false }),
//             '-' => Some(MapCell { coords: p, east: true, west: true, north: false, south: false, start: false }),
//             'L' => Some(MapCell { coords: p, east: true, west: false, north: true, south: false, start: false }),
//             'J' => Some(MapCell { coords: p, east: false, west: true, north: true, south: false, start: false }),
//             '7' => Some(MapCell { coords: p, east: false, west: true, north: false, south: true, start: false }),
//             'F' => Some(MapCell { coords: p, east: true, west: false, north: false, south: true, start: false }),
//             'S' => Some(MapCell { coords: p, east: false, west: false, north: false, south: false, start: true }),
//             _ => None
//         }
//     }
// }

// fn parse_map(input: &str) -> HashMap<Point, MapCell> {
//     let mut map: HashMap<Point, MapCell> = HashMap::new();
//     input.lines().enumerate().for_each(|(y, l)| {
//         l.chars().enumerate().for_each(|(x, c)| {
//             let p = Point { x, y };
//             match MapCell::from_map_char(x, y, c) {
//                 Some(m) => { map.insert(p, m); },
//                 None => ()
//             }
//         });
//     });
//     map
// }

// enum Direction {
//     North,
//     South,
//     East,
//     West
// }

// impl Direction {
//     fn opposite(&self) -> Self {
//         match self {
//             Direction::North => Direction::South,
//             Direction::East => Direction::West,
//             Direction::South => Direction::North,
//             Direction::West => Direction::East
//         }
//     }
// }

// fn get_connections(curr_cell: &MapCell, map: &HashMap<Point, MapCell>) -> Vec<Point> {
//     let mut possibilities: Vec<(Point, Direction)> = Vec::new();
//     let x = curr_cell.coords.x;
//     let y = curr_cell.coords.y;
//     if x != 0 {
//         possibilities.push((Point { x: x-1, y }, Direction::West));
//     }
//     if y != 0 {
//         possibilities.push((Point { x, y: y-1 }, Direction::North));
//     }
//     possibilities.push((Point { x: x+1, y }, Direction::East));
//     possibilities.push((Point { x, y: y+1 }, Direction::South));

//     possibilities.iter().filter_map(|(p, d)| {
//         map.get(p).and_then(|v| {
//             match d.opposite() {
//                 Direction::East => if v.east && (curr_cell.west || curr_cell.start) { Some(*p) } else { None },
//                 Direction::West => if v.west && (curr_cell.east || curr_cell.start) { Some(*p) } else { None },
//                 Direction::North => if v.north && (curr_cell.south || curr_cell.start) { Some(*p) } else { None },
//                 Direction::South => if v.south && (curr_cell.north || curr_cell.start) { Some(*p) } else { None },
//             }
//         })
//     }).collect()
// }

// fn draw_loop(map: &HashMap<Point, MapCell>) -> Vec<&MapCell> {
//     let start = map.values().find(|v| v.start).unwrap();
//     let mut next_cells = get_connections(start, map);
//     let mut loop_points: HashSet<Point> = HashSet::new();
//     loop_points.insert(start.coords);
//     while !next_cells.is_empty() {
//         let mut inserted = 0;
//         next_cells.iter().for_each(|c| {
//             if loop_points.insert(*c) {
//                 inserted += 1;
//             }
//         });
//         if inserted != 0 {
//             next_cells = next_cells.iter().flat_map(|c| {
//                 match map.get(c) {
//                     Some(v) => get_connections(v, map),
//                     None => Vec::new()
//                 }
//             }).collect();
//         } else {
//             next_cells = Vec::new();
//         }
//     }
//     loop_points.iter().filter_map(|p| map.get(p)).collect_vec()
// }

// way too slow. todo!()
pub fn part_one(_input: &str) -> Option<usize> {
    None
    // let map = parse_map(input);
    // let loop_points = draw_loop(&map);
    // let half_len = loop_points.len() / 2;
    // Some(half_len)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
