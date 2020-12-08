use std::str::FromStr;

use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res, opt, recognize},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Insn {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut insns = parse(input).unwrap().1;
    let mut i = 0;
    let mut acc = 0;
    loop {
        let insn = &mut insns[i as usize];
        if insn.1 > 0 {
            break acc;
        }
        insn.1 += 1;
        match insn.0 {
            Insn::Acc(arg) => acc += arg,
            Insn::Jmp(arg) => i += arg - 1,
            _ => {}
        }
        i += 1;
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Insn, u32)>> {
    many0(terminated(map(insn, |insn| (insn, 0)), opt(line_ending)))(input)
}

fn insn(input: &str) -> IResult<&str, Insn> {
    map(
        tuple((
            terminated(alt((tag("acc"), tag("jmp"), tag("nop"))), char(' ')),
            decimal,
        )),
        |(insn, arg)| match insn {
            "acc" => Insn::Acc(arg),
            "jmp" => Insn::Jmp(arg),
            "nop" => Insn::Nop(arg),
            _ => unreachable!(),
        },
    )(input)
}

fn decimal<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(
        recognize(tuple((opt(alt((char('+'), char('-')))), digit1))),
        |num: &str| num.parse(),
    )(input)
}
