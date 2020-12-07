use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| group.bytes()
             .filter(|b| b.is_ascii_lowercase())
             .collect::<HashSet<_>>()
             .len())
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input.split("\n\n")
        .map(|group| group.as_bytes()
             .split(|&b| b == b'\n')
             .fold(0x3FFFFFF, |all, ans| all & ans.iter().fold(0u32, |acc, b| acc | 1 << b - b'a'))
             .count_ones())
        .sum()
}
