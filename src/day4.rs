use aoc_runner_derive::aoc;

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
                    break'passport;
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
