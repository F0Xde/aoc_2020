use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn solve_part1(input: &[u8]) -> u32 {
    parse(input).max().unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[u8]) -> u32 {
    let mut sum = 0;
    let mut min = 1024;
    let mut max = 0;
    for id in parse(input) {
        sum += id;
        min = min.min(id);
        max = max.max(id);
    }

    let expected_sum = (max - min + 1) * (max + min) / 2;
    expected_sum - sum
}

pub fn parse<'a>(input: &'a [u8]) -> impl Iterator<Item = u32> + 'a {
    input.chunks(11).map(|line| {
        id(&line[..10])
    })
}

fn id(seat: &[u8]) -> u32 {
    seat.iter()
        .enumerate()
        .fold(0, |id, (idx, b)| id | ((*b == b'B' || *b == b'R') as u32) << 9 - idx)
}
