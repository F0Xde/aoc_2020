use std::collections::HashMap;

use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    character::complete::newline,
    character::complete::{alpha1, char},
    combinator::map_res,
    combinator::{map, opt, recognize},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> u32 {
    println!("{:#?}", parse(input));
    0
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<(&str, u32)>>> {
    many1(terminated(bag, opt(newline)))(input)
        .map(|(input, bags)| (input, bags.into_iter().collect()))
}

fn bag(input: &str) -> IResult<&str, (&str, Vec<(&str, u32)>)> {
    tuple((
        bag_color,
        tag(" contain "),
        alt((
            map(tag("no other bags."), |_| Vec::with_capacity(0)),
            many1(terminated(contained_bag, alt((tag("."), tag(", "))))),
        )),
    ))(input)
    .map(|(input, (name, _, contained))| (input, (name, contained)))
}

fn contained_bag(input: &str) -> IResult<&str, (&str, u32)> {
    tuple((
        map_res(digit1, |num: &str| num.parse()),
        char(' '),
        bag_color,
    ))(input)
    .map(|(input, (count, _, color))| (input, (color, count)))
}

fn bag_color(input: &str) -> IResult<&str, &str> {
    tuple((
        recognize(tuple((alpha1, char(' '), alpha1))),
        tag(" bag"),
        opt(char('s')),
    ))(input)
    .map(|(input, (name, ..))| (input, name))
}
