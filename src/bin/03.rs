advent_of_code::solution!(3);

use anyhow::Result;
use regex::Regex;

enum LineCellVal {
    Number(u32),
    Symbol(char),
    Empty,
}

struct LineCell {
    val: LineCellVal,
    cols: Vec<u32>,
    row: u32,
}

fn parse_line(line: &str, row: u32) -> Result<Vec<LineCell>> {
    let mut res: Vec<LineCell> = vec![];
    let re = Regex::new(r"(\d+)")?;
    re.find_iter(line).for_each(|ma| {
        let start = ma.start();
        let len = ma.len();
        let val = ma.as_str().parse::<u32>().unwrap();
        let cols = (start..(start + len))
            .map(|x| x.try_into().unwrap())
            .collect();
        res.push(LineCell {
            val: LineCellVal::Number(val),
            cols,
            row,
        })
    });
    line.char_indices().for_each(|(ix, c)| {
        let col = ix.try_into().unwrap();
        if c == '.' {
            res.push(LineCell {
                val: LineCellVal::Empty,
                cols: vec![col],
                row,
            })
        } else if !c.is_numeric() {
            res.push(LineCell {
                val: LineCellVal::Symbol(c),
                cols: vec![col],
                row,
            })
        }
    });
    Ok(res)
}

fn has_adj_symbol(row: u32, cols: Vec<u32>, cells: &[LineCell]) -> bool {
    let min_minus_1 = if cols[0] == 0 { 0 } else { cols[0] - 1 };
    let max_plus_1 = cols[cols.len() - 1] + 1;
    let up: Vec<(u32, u32)> = if row == 0 {
        vec![]
    } else {
        (min_minus_1..=max_plus_1).map(|i| (row - 1, i)).collect()
    };
    let down: Vec<(u32, u32)> = (min_minus_1..=max_plus_1).map(|i| (row + 1, i)).collect();
    let same = vec![(row, min_minus_1), (row, max_plus_1)];
    let all_adj = [up, down, same].concat();
    all_adj.iter().any(|(row, col)| {
        cells.iter().any(|l| match l.val {
            LineCellVal::Empty => false,
            LineCellVal::Number(_) => false,
            LineCellVal::Symbol(_) => l.row == *row && l.cols.contains(col),
        })
    })
}

// only Some((x,y)) if there are exactly 2 adjacent nums
fn get_adj_nums(row: u32, col: u32, cells: &[LineCell]) -> Option<(u32, u32)> {
    let min_minus_1 = if col == 0 { 0 } else { col - 1 };
    let max_plus_1 = col + 1;
    let up: Vec<(u32, u32)> = if row == 0 {
        vec![]
    } else {
        (min_minus_1..=max_plus_1).map(|i| (row - 1, i)).collect()
    };
    let down: Vec<(u32, u32)> = (min_minus_1..=max_plus_1).map(|i| (row + 1, i)).collect();
    let same = vec![(row, min_minus_1), (row, max_plus_1)];
    let all_adj = [up, down, same].concat();
    let adj_nums: Vec<u32> = cells
        .iter()
        .filter_map(|c| match c.val {
            LineCellVal::Number(n) => {
                if all_adj
                    .iter()
                    .any(|(r1, c1)| r1 == &c.row && c.cols.contains(c1))
                {
                    Some(n)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    if adj_nums.len() == 2 {
        Some((adj_nums[0], adj_nums[1]))
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let all_cells: Vec<LineCell> = input
        .lines()
        .enumerate()
        .flat_map(|(ix, l)| parse_line(l, ix.try_into().unwrap()).unwrap())
        .collect();
    let nums: u32 = all_cells
        .iter()
        .filter_map(|c| match c.val {
            LineCellVal::Empty => None,
            LineCellVal::Symbol(_) => None,
            LineCellVal::Number(n) => has_adj_symbol(c.row, c.cols.clone(), &all_cells).then_some(n),
        })
        .sum();
    Some(nums)
}

pub fn part_two(input: &str) -> Option<u32> {
    let all_cells: Vec<LineCell> = input
        .lines()
        .enumerate()
        .flat_map(|(ix, l)| parse_line(l, ix.try_into().unwrap()).unwrap())
        .collect();
    let matches = all_cells
        .iter()
        .filter_map(|c| match c.val {
            LineCellVal::Symbol('*') => get_adj_nums(c.row, c.cols[0], &all_cells),
            _ => None,
        })
        .map(|(a, b)| a * b)
        .sum();
    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
