mod day;
pub mod template;

pub use day::*;

use std::hash::Hash;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
