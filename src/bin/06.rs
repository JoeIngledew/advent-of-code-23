advent_of_code::solution!(6);

struct RResult {
    time: u64,
    distance: u64,
}

impl RResult {
    fn get_possibilities(&self) -> Vec<u64> {
        // 0..n..x, where n * (x-n) > d
        (1..self.time)
            .filter(|x| *x * (self.time - x) > self.distance).collect()
    }
}

fn parse_input_2(input: &str) -> RResult {
    let nums: Vec<u64> = input.lines().map(|l| {
        let nums: String = l.replace(|c: char| !c.is_numeric(), "");
        nums.parse::<u64>().unwrap()
    }).collect();
    RResult { time: nums[0], distance: nums[1] }
}

fn parse_input(input: &str) -> Vec<RResult> {
    let lines = input.lines();
    let num_lines: Vec<Vec<u64>> = lines
        .map(|l| l.split(' ').filter_map(|s| s.parse::<u64>().ok()).collect())
        .collect();
    num_lines[0]
        .iter()
        .zip(&num_lines[1])
        .map(|(t, d)| RResult {
            time: *t,
            distance: *d,
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = parse_input(input)
            .iter()
            .map(|x| x.get_possibilities().len())
            .fold(1, |acc, curr| acc * curr);
    Some(res.try_into().unwrap())
}

// this is a bit slow
pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input_2(input).get_possibilities().len().try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
