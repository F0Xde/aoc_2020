use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

pub struct Password {
    pub letter: u8,
    pub range: RangeInclusive<usize>,
    pub text: String,
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Password> {
    input.lines().map(|line| {
        let (start, rest) = line.split_once('-').unwrap();
        let (end, rest) = rest.split_once(' ').unwrap();
        let range = start.parse().unwrap()..=end.parse().unwrap();
        let letter = rest.as_bytes()[0];
        let (_, text) = rest.split_once(' ').unwrap();
        Password {
            letter,
            range,
            text: text.to_string()
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {
    input.iter().fold(0, |valid, pw| {
        let mut found: usize = 0;
        for &c in pw.text.as_bytes() {
            if c == pw.letter {
                found += 1;
            }
        }
        valid + pw.range.contains(&found) as usize
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {
    input.iter().fold(0, |valid, pw| {
        let bytes = pw.text.as_bytes();
        valid + 
            ((bytes[pw.range.start() - 1] == pw.letter) as usize ^
            (bytes[pw.range.end() - 1] == pw.letter) as usize)
    })
}
