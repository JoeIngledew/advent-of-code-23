advent_of_code::solution!(2);

use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
enum Day2Err {
    ParseLineError,
    BadRegex,
    BadInput,
}

#[derive(Debug)]
struct GamePart1 {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl GamePart1 {
    fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
        red >= self.max_red && green >= self.max_green && blue >= self.max_blue
    }

    fn calc_power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

impl Display for GamePart1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, red: {}, green: {}, blue: {}",
            self.id, self.max_red, self.max_green, self.max_blue
        )
    }
}

fn parse_input_line(line: &str) -> Result<GamePart1, Day2Err> {
    let (game, rest) = line.split_once(':').ok_or(Day2Err::ParseLineError)?;
    let game_id = game
        .matches(char::is_numeric)
        .collect::<Vec<_>>()
        .join("")
        .parse::<u32>()
        .map_err(|_| Day2Err::ParseLineError)?;
    let re = Regex::new(r"(\d+) (blue|red|green)").map_err(|_| Day2Err::BadRegex)?;
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;
    for (_, [count, color]) in re.captures_iter(rest).map(|c| c.extract()) {
        match color {
            "red" => {
                max_red = max_red.max(count.parse::<u32>().map_err(|_| Day2Err::ParseLineError)?)
            }
            "green" => {
                max_green =
                    max_green.max(count.parse::<u32>().map_err(|_| Day2Err::ParseLineError)?)
            }
            "blue" => {
                max_blue = max_blue.max(count.parse::<u32>().map_err(|_| Day2Err::ParseLineError)?)
            }
            _ => return Err(Day2Err::BadInput),
        }
    }
    Ok(GamePart1 {
        id: game_id,
        max_red,
        max_green,
        max_blue,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter_map(|l| {
            parse_input_line(l)
                .map(|g| if g.possible(12, 13, 14) { g.id } else { 0 })
                .map_or_else(|_| None, Some)
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .filter_map(|l| parse_input_line(l).map_or_else(|_| None, |g| Some(g.calc_power())))
        .sum();
    Some(res)
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
        assert_eq!(result, Some(2286));
    }
}
