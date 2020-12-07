use std::ops::RangeInclusive;

use aoc_runner_derive::aoc;

macro_rules! unwrap_or_false {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return false,
        }
    };
}

const BYR: u8 = 0b00000001;
const IYR: u8 = 0b00000010;
const EYR: u8 = 0b00000100;
const HGT: u8 = 0b00001000;
const HCL: u8 = 0b00010000;
const ECL: u8 = 0b00100000;
const PID: u8 = 0b01000000;
const VALID: u8 = BYR | IYR | EYR | HGT | HCL | ECL | PID;

#[aoc(day4, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    let mut valid = 0;
    let mut pos = 0;
    loop {
        if pos >= input.len() {
            break;
        }
        let mut fields = 0;
        'passport: loop {
            let key = &input[pos..pos + 3];
            match key {
                b"byr" => fields |= BYR,
                b"iyr" => fields |= IYR,
                b"eyr" => fields |= EYR,
                b"hgt" => fields |= HGT,
                b"hcl" => fields |= HCL,
                b"ecl" => fields |= ECL,
                b"pid" => fields |= PID,
                _ => {}
            }
            pos += 4;
            while input[pos] != b' ' && input[pos] != b'\n' {
                pos += 1;
                if pos >= input.len() {
                    break 'passport;
                }
            }
            pos += 1;
            if input[pos] == b'\n' {
                pos += 1;
                break;
            }
        }
        if (fields & VALID) == VALID {
            valid += 1;
        }
    }
    valid
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|passport| {
            passport
                .split(|c| c == ' ' || c == '\n')
                .filter(|&field| {
                    let name = &field[..3];
                    let value = &field[4..];
                    match name {
                        "byr" => is_four_digit_num(value, 1920..=2002),
                        "iyr" => is_four_digit_num(value, 2010..=2020),
                        "eyr" => is_four_digit_num(value, 2020..=2030),
                        "hgt" => {
                            let (num, suffix) = value.split_at(value.len() - 2);
                            match suffix {
                                "cm" => 150..=193,
                                "in" => 59..=76,
                                _ => return false,
                            }
                            .contains(&unwrap_or_false!(num.parse::<u32>()))
                        }
                        "hcl" => {
                            value.as_bytes()[0] == b'#'
                                && value[1..].as_bytes().iter().all(|b| b.is_ascii_hexdigit())
                        }
                        "ecl" => {
                            matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
                        }
                        "pid" => {
                            value.len() == 9 && value.as_bytes().iter().all(|b| b.is_ascii_digit())
                        }
                        _ => false,
                    }
                })
                .count()
                == 7
        })
        .count()
}

fn is_four_digit_num(input: &str, range: RangeInclusive<u32>) -> bool {
    input.len() == 4 && range.contains(&unwrap_or_false!(input.parse()))
}
