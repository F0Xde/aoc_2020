use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
pub fn solve_part1(input: &str) -> usize {
    input.split("\n\n")
        .map(|group| group.bytes()
             .filter(|b| b.is_ascii_lowercase())
             .collect::<HashSet<_>>().len())
        .sum()
}
