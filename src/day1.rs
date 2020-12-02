use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[i32]) -> i32 {
    for (idx, a) in input.iter().enumerate() {
        for (_, b) in input.iter().enumerate().skip(idx) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!();
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[i32]) -> i32 {
    for (idx, a) in input.iter().enumerate() {
        for (idx, b) in input.iter().enumerate().skip(idx) {
            for (_, c) in input.iter().enumerate().skip(idx) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!();
}
