advent_of_code::solution!(1);

#[derive(Debug)]
enum Day1Err {
    NoCharacters,
    ParseErr,
}

fn take_digits(s: &str) -> Result<u32, Day1Err> {
    let line_digits: Vec<char> = String::from(s)
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    let first = line_digits.first().ok_or(Day1Err::NoCharacters)?;
    let last = line_digits.last().ok_or(Day1Err::NoCharacters)?;
    let chars = vec![first, last];
    let final_str: String = chars.into_iter().collect();
    final_str.parse::<u32>().map_err(|_| Day1Err::ParseErr)
}

fn process(input: &str) -> u32 {
    input.lines().map(|l| take_digits(l).unwrap()).sum()
}

fn take_digits_with_spelled_nums(s: &str) -> Result<u32, Day1Err> {
    let replaced_str = s
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine"); // keep the letters either side just in case the letters can be used in multiple numbers :)
    take_digits(replaced_str.as_str())
}

fn process_p2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| take_digits_with_spelled_nums(l).unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = process(input);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let res = process_p2(input);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
