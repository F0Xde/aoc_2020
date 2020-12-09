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

enum Res {
    Done(i32),
    Loop(i32),
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut insns = parse(input).unwrap().1;
    if let Res::Loop(acc) = run(&mut insns, None) {
        acc
    } else {
        unreachable!();
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> i32 {
    let mut insns = parse(input).unwrap().1;
    for change in 0.. {
        if let Res::Done(acc) = run(&mut insns, Some(change)) {
            return acc;
        }
    }
    unreachable!();
}

fn run(insns: &mut [(Insn, bool)], swap: Option<u32>) -> Res {
    for (_, visited) in insns.iter_mut() {
        *visited = false;
    }
    let mut i = 0;
    let mut acc = 0;
    let mut swap_idx = 0;
    loop {
        let (insn, visited) = &mut insns[i as usize];
        if *visited {
            break Res::Loop(acc);
        }
        *visited = true;

        let swapped = match swap {
            Some(swap) if swap == swap_idx => {
                swap_insn(insn);
                true
            }
            _ => false,
        };
        match &insn {
            Insn::Acc(arg) => acc += arg,
            Insn::Jmp(arg) => {
                swap_idx += 1;
                i += arg - 1
            }
            _ => swap_idx += 1,
        }
        if swapped {
            swap_insn(insn);
        }
        i += 1;
        if i as usize >= insns.len() {
            break Res::Done(acc);
        }
    }
}

fn swap_insn(insn: &mut Insn) {
    match insn {
        Insn::Jmp(arg) => *insn = Insn::Nop(*arg),
        Insn::Nop(arg) => *insn = Insn::Jmp(*arg),
        _ => {}
    }
}

fn parse(input: &str) -> IResult<&str, Vec<(Insn, bool)>> {
    many0(terminated(
        map(insn, |insn| (insn, false)),
        opt(line_ending),
    ))(input)
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
