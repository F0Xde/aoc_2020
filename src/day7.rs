use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};
use std::collections::HashMap;

const SHINY_GOLD: &str = "shiny gold";

#[aoc(day7, part1)]
pub fn solve_part1(input: &str) -> usize {
    let bags = parse(input).unwrap().1;
    bags.iter()
        .filter(|(&name, _)| contains_sg(name, &bags, &mut Vec::new()))
        .count()
        - 1
}

fn contains_sg<'a>(
    name: &'a str,
    bags: &HashMap<&'a str, Vec<(&'a str, u32)>>,
    visited: &mut Vec<&'a str>,
) -> bool {
    if name == SHINY_GOLD {
        true
    } else if visited.contains(&name) {
        false
    } else {
        visited.push(name);
        for (child, _) in &bags[name] {
            if contains_sg(child, bags, visited) {
                return true;
            }
        }
        false
    }
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let bags = parse(input).unwrap().1;
    child_count(SHINY_GOLD, &bags)
}

fn child_count(name: &str, bags: &HashMap<&str, Vec<(&str, u32)>>) -> u32 {
    bags[name]
        .iter()
        .map(|(child, count)| count * (child_count(child, bags) + 1))
        .sum()
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Vec<(&str, u32)>>> {
    many1(terminated(bag, opt(line_ending)))(input)
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
