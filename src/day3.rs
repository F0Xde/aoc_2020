use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u8]) -> usize {
    check_slope(input, get_width(input), (3, 1))
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[u8]) -> usize {
    let width = get_width(input);
    let mut result = 1;
    for x in (1..=7).step_by(2) {
        result *= check_slope(input, width, (x, 1));
    }
    result * check_slope(input, width, (1, 2))
}

/// Returns the width of the map corresponding to the given [input] without a newline
fn get_width(input: &[u8]) -> usize {
    input.iter().position(|&b| b == b'\n').unwrap()
}

fn check_slope(input: &[u8], width: usize, slope: (usize, usize)) -> usize {
    let mut trees = 0;
    let mut x = slope.0;
    let mut y = slope.1;
    return loop {
        match input.get(y * (width + 1) + (x % width)) {
            Some(&chr) => trees += (chr == b'#') as usize,
            None => break trees,
        }
        x += slope.0;
        y += slope.1;
    }
}
