use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum MapType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location,
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

fn parse_instructions_2(input: &str) -> Instructions2 {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds_unparsed: Vec<u64> = seeds_line.split_once(": ").map_or(vec![], |l| {
        l.1.split(char::is_whitespace)
            .filter_map(|x| match x {
                "" => None,
                s => s.parse::<u64>().ok(),
            })
            .collect()
    });
    let seeds: Vec<u64> = seeds_unparsed
        .chunks(2)
        .map(|chunk| {
            let start = *chunk.first().unwrap();
            let range = *chunk.last().unwrap();
            let r: Vec<u64> = (start..(start + range)).collect();
            r
        })
        .collect::<Vec<Vec<u64>>>()
        .concat();

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

pub fn part_one(input: &str) -> Option<u64> {
    let instructs = parse_instructions(input);
    let mut final_locations = instructs.get_final_locations_2();
    final_locations.sort();
    final_locations.first().copied()
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructs = parse_instructions_2(input);
    let mut final_locations = instructs.get_final_locations_2();
    final_locations.sort();
    final_locations.first().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

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
