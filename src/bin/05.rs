use std::{collections::BTreeSet, str::FromStr};

advent_of_code::solution!(5);

// PART 2 CODE - https://gist.github.com/shaansheikh/bbda4b79a0fe5a32a484f66fb6cd0cd4
struct Day5Err;

struct Interval {
    start: i64,
    end: i64,
    offset: i64,
}

impl FromStr for Interval {
    type Err = Day5Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split(' ').map(|x| x.parse::<i64>());
        let dest = nums.next().unwrap().map_err(|_| Day5Err)?;
        let start = nums.next().unwrap().map_err(|_| Day5Err)?;
        let len = nums.next().unwrap().map_err(|_| Day5Err)?;
        let end = start + len - 1;
        let offset = dest - start;
        Ok(Interval { end, start, offset })
    }
}

impl Interval {
    fn new(start: i64, len: i64) -> Self {
        Interval {
            start,
            end: start + len + 1,
            offset: 0,
        }
    }

    fn contains(&self, point: i64) -> bool {
        self.start <= point && point <= self.end
    }

    fn map(&self, point: i64) -> i64 {
        point + self.offset
    }

    fn outputs(&self, mapped_point: i64) -> bool {
        self.start + self.offset <= mapped_point && mapped_point <= self.end + self.offset
    }

    fn undo_map(&self, mapped_point: i64) -> i64 {
        mapped_point - self.offset
    }
}

struct IntervalList(Vec<Interval>);

impl IntervalList {
    fn process(&self, point: i64) -> i64 {
        for i in &self.0 {
            if i.contains(point) {
                return i.map(point);
            }
        }
        point
    }

    fn undo_proc(&self, outputs: &mut BTreeSet<i64>) -> BTreeSet<i64> {
        let mut potentials: BTreeSet<i64> = BTreeSet::new();
        for i in &self.0 {
            for o in outputs.iter() {
                if i.outputs(*o) {
                    potentials.insert(i.undo_map(*o));
                }
            }
        }
        outputs.append(&mut potentials);
        outputs.to_owned()
    }

    fn filter(&self, points: BTreeSet<i64>) -> BTreeSet<i64> {
        let mut res: BTreeSet<i64> = BTreeSet::new();
        for p in points.iter() {
            if self.0.iter().any(|i| i.contains(*p)) {
                res.insert(*p);
            }
        }
        res
    }

    fn boundaries(&self, candidate_points: &mut BTreeSet<i64>) -> BTreeSet<i64> {
        for i in self.0.iter() {
            candidate_points.insert(i.start);
            candidate_points.insert(i.end);
        }
        candidate_points.to_owned()
    }
}

fn parse(input: &str) -> (IntervalList, Vec<IntervalList>) {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds_unparsed: Vec<i64> = seeds_line.split_once(": ").map_or(vec![], |l| {
        l.1.split(char::is_whitespace)
            .filter_map(|x| match x {
                "" => None,
                s => s.parse::<i64>().ok(),
            })
            .collect()
    });
    let seeds_intervals: Vec<Interval> = seeds_unparsed
        .chunks(2)
        .map(|chunk| {
            let start = *chunk.first().unwrap();
            let range = *chunk.last().unwrap();
            Interval::new(start, range)
        })
        .collect::<Vec<Interval>>();
    let seeds_intervals = IntervalList(seeds_intervals);

    let mut curr: Vec<Interval> = Vec::new();
    let mut int_lists: Vec<IntervalList> = Vec::new();
    for l in lines {
        if l.is_empty() {
            if !curr.is_empty() {
                int_lists.push(IntervalList(curr));
            }
            curr = Vec::new();
        } else if let Ok(i) = l.parse::<Interval>() {
            curr.push(i);
        }
    }

    (seeds_intervals, int_lists)
}

fn do_part_two(input: &str) -> i64 {
    let (seed_ints, mut ivs) = parse(input);
    let mut candidate_set: BTreeSet<i64> = BTreeSet::new();
    ivs.reverse();
    for i in &ivs {
        candidate_set = i.undo_proc(&mut candidate_set);
        candidate_set = i.boundaries(&mut candidate_set);
    }

    candidate_set = seed_ints.filter(candidate_set);

    let mut curr = i64::MAX;

    ivs.reverse();
    for s in candidate_set.iter() {
        let mut temp = *s;
        for i in ivs.iter() {
            temp = i.process(temp);
        }
        if temp < curr {
            curr = temp
        }
    }
    curr
}

// END PART 2 CODE

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
enum MapType {
    Seed = 7,
    Soil = 6,
    Fertilizer = 5,
    Water = 4,
    Light = 3,
    Temp = 2,
    Humidity = 1,
    Location = 0,
}

struct ParseMapTypeErr;

impl FromStr for MapType {
    type Err = ParseMapTypeErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "seed" => Ok(MapType::Seed),
            "soil" => Ok(MapType::Soil),
            "fertilizer" => Ok(MapType::Fertilizer),
            "water" => Ok(MapType::Water),
            "light" => Ok(MapType::Light),
            "temperature" => Ok(MapType::Temp),
            "humidity" => Ok(MapType::Humidity),
            "location" => Ok(MapType::Location),
            _ => Err(ParseMapTypeErr),
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct SMap2 {
    source_start: u64,
    dest_start: u64,
    range: u64,
}

impl SMap2 {
    fn run_map(&self, input: u64) -> u64 {
        let range_upper_bound_excl = self.source_start + self.range;
        if range_upper_bound_excl > input && self.source_start <= input {
            // dest 50, source 55, 55 -> 50, -5, dest - source
            let s: i64 = self.source_start.try_into().unwrap();
            let d: i64 = self.dest_start.try_into().unwrap();
            let diff: i64 = d - s;
            let i: i64 = input.try_into().unwrap();
            return (i + diff).try_into().unwrap();
        }
        input
    }
}

struct Instructions2 {
    seeds: Vec<u64>,
    seed_to_soil: Vec<SMap2>,
    soil_to_fert: Vec<SMap2>,
    fert_to_water: Vec<SMap2>,
    water_to_light: Vec<SMap2>,
    light_to_temp: Vec<SMap2>,
    temp_to_hum: Vec<SMap2>,
    hum_to_loc: Vec<SMap2>,
}

fn get_next(xs: &[SMap2], curr: u64) -> u64 {
    xs.iter()
        .find_map(|x| {
            let y = x.run_map(curr);
            if y != curr {
                Some(y)
            } else {
                None
            }
        })
        .unwrap_or(curr)
}

impl Instructions2 {
    fn new(seeds: Vec<u64>) -> Self {
        Instructions2 {
            seeds,
            seed_to_soil: vec![],
            soil_to_fert: vec![],
            fert_to_water: vec![],
            water_to_light: vec![],
            light_to_temp: vec![],
            temp_to_hum: vec![],
            hum_to_loc: vec![],
        }
    }

    fn get_final_locations_2(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|s| {
                let mut curr = *s;
                curr = get_next(&self.seed_to_soil, curr);
                curr = get_next(&self.soil_to_fert, curr);
                curr = get_next(&self.fert_to_water, curr);
                curr = get_next(&self.water_to_light, curr);
                curr = get_next(&self.light_to_temp, curr);
                curr = get_next(&self.temp_to_hum, curr);
                get_next(&self.hum_to_loc, curr)
            })
            .collect()
    }

    // fn get_min_final_loc(&self) -> Option<u64> {
    //     let ranges_loc: Vec<TypedRange> = self.hum_to_loc.iter().map(|s| TypedRange::new(MapType::Location, s.clone())).collect();
    //     let ranges_hum: Vec<TypedRange> = self.temp_to_hum.iter().map(|s| TypedRange::new(MapType::Humidity, s.clone())).collect();
    //     let ranges_temp: Vec<TypedRange> = self.light_to_temp.iter().map(|s| TypedRange::new(MapType::Temp, s.clone())).collect();
    //     let ranges_light: Vec<TypedRange> = self.water_to_light.iter().map(|s| TypedRange::new(MapType::Light, s.clone())).collect();
    //     let ranges_wat: Vec<TypedRange> = self.fert_to_water.iter().map(|s| TypedRange::new(MapType::Water, s.clone())).collect();
    //     let ranges_fert: Vec<TypedRange> = self.soil_to_fert.iter().map(|s| TypedRange::new(MapType::Fertilizer, s.clone())).collect();
    //     let ranges_soil: Vec<TypedRange> = self.seed_to_soil.iter().map(|s| TypedRange::new(MapType::Soil, s.clone())).collect();
    //     let mut all_ranges: Vec<TypedRange> = [ranges_loc, ranges_hum, ranges_temp, ranges_light, ranges_wat, ranges_fert, ranges_soil].concat();
    //     all_ranges.sort();
    //     let mut iterator = all_ranges.iter();

    //     let mut res: Option<u64> = None;
    //     while res.is_none() {
    //         let next = iterator.next();
    //         if next.is_none() {
    //             break;
    //         }

    //         res = test_option(next.unwrap(), &self);
    //     }

    //     res
    // }
}

fn parse_from_to(l: &str) -> Option<(MapType, MapType)> {
    l.split_once(' ')
        .and_then(|x| x.0.split_once("-to-"))
        .and_then(
            |(from, to)| match (from.parse::<MapType>(), to.parse::<MapType>()) {
                (Ok(f), Ok(t)) => Some((f, t)),
                _ => None,
            },
        )
}

fn parse_map_line_2(l: &str) -> SMap2 {
    let nums: Vec<u64> = l
        .split(' ')
        .filter_map(|x| match x {
            "" => None,
            x => x.parse::<u64>().ok(),
        })
        .collect();
    SMap2 {
        source_start: nums[1],
        dest_start: nums[0],
        range: nums[2],
    }
}

fn parse_instructions(input: &str) -> Instructions2 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds: Vec<u64> = seeds_line.split_once(": ").map_or(vec![], |l| {
        l.1.split(char::is_whitespace)
            .filter_map(|x| match x {
                "" => None,
                s => s.parse::<u64>().ok(),
            })
            .collect()
    });

    let mut curr_map: Option<(MapType, MapType)> = None;
    let mut builder = Instructions2::new(seeds);

    for next_line in lines {
        let is_nums = next_line.starts_with(char::is_numeric);
        let is_spec_line = next_line.starts_with(char::is_alphabetic);

        match (&curr_map, is_nums, is_spec_line) {
            (Some(_), false, false) => curr_map = None,
            (None, false, true) => {
                curr_map = parse_from_to(next_line);
            }
            (Some((MapType::Seed, MapType::Soil)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.seed_to_soil.push(m);
            }
            (Some((MapType::Soil, MapType::Fertilizer)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.soil_to_fert.push(m);
            }
            (Some((MapType::Fertilizer, MapType::Water)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.fert_to_water.push(m);
            }
            (Some((MapType::Water, MapType::Light)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.water_to_light.push(m);
            }
            (Some((MapType::Light, MapType::Temp)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.light_to_temp.push(m);
            }
            (Some((MapType::Temp, MapType::Humidity)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.temp_to_hum.push(m);
            }
            (Some((MapType::Humidity, MapType::Location)), true, false) => {
                let m = parse_map_line_2(next_line);
                builder.hum_to_loc.push(m);
            }
            _ => (),
        }
    }

    builder
}

// fn parse_instructions_2(input: &str) -> Instructions2 {
//     let mut lines = input.lines();
//     let seeds_line = lines.next().unwrap();
//     let seeds_unparsed: Vec<u64> = seeds_line.split_once(": ").map_or(vec![], |l| {
//         l.1.split(char::is_whitespace)
//             .filter_map(|x| match x {
//                 "" => None,
//                 s => s.parse::<u64>().ok(),
//             })
//             .collect()
//     });
//     let seeds: Vec<u64> = seeds_unparsed
//         .chunks(2)
//         .map(|chunk| {
//             let start = *chunk.first().unwrap();
//             let range = *chunk.last().unwrap();
//             let r: Vec<u64> = (start..(start + range)).collect();
//             r
//         })
//         .collect::<Vec<Vec<u64>>>()
//         .concat();

//     let mut curr_map: Option<(MapType, MapType)> = None;
//     let mut builder = Instructions2::new(seeds);

//     for next_line in lines {
//         let is_nums = next_line.starts_with(char::is_numeric);
//         let is_spec_line = next_line.starts_with(char::is_alphabetic);

//         match (&curr_map, is_nums, is_spec_line) {
//             (Some(_), false, false) => curr_map = None,
//             (None, false, true) => {
//                 curr_map = parse_from_to(next_line);
//             }
//             (Some((MapType::Seed, MapType::Soil)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.seed_to_soil.push(m);
//             }
//             (Some((MapType::Soil, MapType::Fertilizer)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.soil_to_fert.push(m);
//             }
//             (Some((MapType::Fertilizer, MapType::Water)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.fert_to_water.push(m);
//             }
//             (Some((MapType::Water, MapType::Light)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.water_to_light.push(m);
//             }
//             (Some((MapType::Light, MapType::Temp)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.light_to_temp.push(m);
//             }
//             (Some((MapType::Temp, MapType::Humidity)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.temp_to_hum.push(m);
//             }
//             (Some((MapType::Humidity, MapType::Location)), true, false) => {
//                 let m = parse_map_line_2(next_line);
//                 builder.hum_to_loc.push(m);
//             }
//             _ => (),
//         }
//     }

//     builder
// }

pub fn part_one(input: &str) -> Option<u64> {
    let instructs = parse_instructions(input);
    let mut final_locations = instructs.get_final_locations_2();
    final_locations.sort();
    final_locations.first().copied()
}

pub fn part_two(input: &str) -> Option<i64> {
    let res = do_part_two(input);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    // i don't get it
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_smap_basic() {
        //50 98 2
        let smap = SMap2 {
            dest_start: 50,
            source_start: 98,
            range: 2,
        };
        let result1 = smap.run_map(98);
        let result2 = smap.run_map(99);
        let result3 = smap.run_map(2);
        let result4 = smap.run_map(100);
        assert_eq!(result1, 50);
        assert_eq!(result2, 51);
        assert_eq!(result3, 2);
        assert_eq!(result4, 100);
    }

    // #[test]
    // fn why_so_slow() {
    //     let input = &advent_of_code::template::read_file("inputs", DAY);
    //     let res = part_one(input);
    //     assert_eq!(res, None);
    // }

    #[test]
    fn test_smap_basic_2() {
        //50 98 2
        let smap_1 = SMap2 {
            source_start: 53,
            dest_start: 49,
            range: 8,
        };
        let smap_2 = SMap2 {
            source_start: 11,
            dest_start: 0,
            range: 42,
        };
        let smap_3 = SMap2 {
            source_start: 0,
            dest_start: 42,
            range: 7,
        };
        let smap_4 = SMap2 {
            source_start: 7,
            dest_start: 57,
            range: 4,
        };
        let result1 = smap_1.run_map(53);
        let result2 = smap_2.run_map(53);
        let result3 = smap_3.run_map(53);
        let result4 = smap_4.run_map(53);
        assert!([result1, result2, result3, result4].contains(&49));
    }

    // #[test]
    // fn test_maps_to_map() {
    //     let maps = vec![
    //         SMap2 { dest_start: 50, source_start: 98, range: 2 },
    //         SMap2 { dest_start: 52, source_start: 50, range: 48 },
    //     ];
    //     let hashmap = maps_to_map(&maps);
    //     assert!(true);
    // }
}
