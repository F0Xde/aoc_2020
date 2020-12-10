use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<u32> {
    let mut input: Vec<_> = input.lines().map(|num| num.parse().unwrap()).collect();
    input.sort_unstable();
    input.insert(0, 0);
    input.push(input.last().unwrap() + 3);
    input
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let (one, three) = input
        .windows(2)
        .fold((0, 0), |(one, three), win| match win[1] - win[0] {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => (one, three),
        });
    one * three
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[u32]) -> u64 {
    let mut paths = vec![0u64; input.len()];
    paths[0] = 1;
    for (idx, current) in input.iter().enumerate() {
        for offset in 1..=3 {
            if let Some(next) = input.get(idx + offset) {
                if next - current <= 3 {
                    paths[idx + offset] += paths[idx];
                }
            }
        }
    }
    *paths.last().unwrap()
}
