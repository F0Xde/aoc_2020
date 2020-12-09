use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
pub fn gen(input: &str) -> Vec<u64> {
    input.lines().map(|num| num.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    *input
        .iter()
        .enumerate()
        .skip(25)
        .find(|(idx, &n)| !is_sum(n, &input[idx - 25..*idx]))
        .unwrap()
        .1
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let invalid = solve_part1(input);
    for n in 2.. {
        if let Some(range) = input
            .windows(n)
            .find(|range| range.iter().sum::<u64>() == invalid)
        {
            return range.iter().max().unwrap() + range.iter().min().unwrap();
        }
    }
    unreachable!();
}

fn is_sum(num: u64, other: &[u64]) -> bool {
    for (idx, n1) in other.iter().enumerate() {
        for n2 in other.iter().skip(idx) {
            if n1 + n2 == num {
                return true;
            }
        }
    }
    false
}
