use std::ops::Range;

use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input.lines().map(seat_id).max().unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut ids: Vec<_> = input.lines().map(seat_id).collect();
    ids.sort_unstable();
    for (idx, id) in ids.iter().enumerate() {
        if id + 2 == ids[idx + 1] {
            return id + 1;
        }
    }
    unreachable!();
}

fn seat_id(seat: &str) -> u32 {
    let (row, col) = seat_pos(seat);
    row * 8 + col
}

fn seat_pos(seat: &str) -> (u32, u32) {
    let bytes = seat.as_bytes();
    (bsp(&bytes[..7], 0..128, b'F', b'B'), bsp(&bytes[7..], 0..8, b'L', b'R'))
}

fn bsp(input: &[u8], init: Range<u32>, lower: u8, upper: u8) -> u32 {
    input.iter().fold(init, |range, &c| {
        let middle = (range.end + range.start) / 2;
        if c == lower {
            range.start..middle
        } else if c == upper {
            middle..range.end
        } else {
            range
        }
    }).start
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEATS: [(&str, u32, u32, u32); 4] = [
        ("FBFBBFFRLR", 44, 5, 357),
        ("BFFFBBFRRR", 70, 7, 567),
        ("FFFBBBFRRR", 14, 7, 119),
        ("BBFFBBFRLL", 102, 4, 820),
    ];

    #[test]
    fn test_seat_id() {
        for (seat, _, _, id) in &SEATS {
            assert_eq!(*id, seat_id(seat));
        }
    }

    #[test]
    fn test_seat_pos() {
        for (seat, row, col, _) in &SEATS {
            assert_eq!((*row, *col), seat_pos(seat));
        }
    }
}
