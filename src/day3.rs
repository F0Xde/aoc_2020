use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    // width without newline
    let width = input.iter().position(|&b| b == b'\n').unwrap();
    let mut trees = 0;
    let mut x = 3;
    let mut y = 1;
    return loop {
        match input.get(y * (width + 1) + (x % width)) {
            Some(&chr) => trees += (chr == b'#') as usize,
            None => break trees,
        }
        x += 3;
        y += 1;
    }
}
