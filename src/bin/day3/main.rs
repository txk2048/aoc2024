use clap::{command, value_parser, Arg};
use std::fs;
use std::path::PathBuf;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{char, i32},
    combinator::{all_consuming, map},
    multi::many0,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

enum ParseInstruction {
    Nop,
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, ParseInstruction> {
    map(
        delimited(tag("mul("), separated_pair(i32, char(','), i32), char(')')),
        |(a, b)| ParseInstruction::Mul(a, b),
    )(input)
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let parse_do = map(tag("do()"), |_| ParseInstruction::Do);
    let parse_dont = map(tag("don't()"), |_| ParseInstruction::Dont);
    let parse_nop = map(take(1usize), |_| ParseInstruction::Nop);
    let parse_instruction = alt((parse_mul, parse_do, parse_dont, parse_nop));

    all_consuming(many0(parse_instruction))(input)
        .finish()
        .map(|(_, instructions)| {
            instructions
                .iter()
                .filter_map(|instr| match instr {
                    ParseInstruction::Nop => None,
                    ParseInstruction::Do => Some(Instruction::Do),
                    ParseInstruction::Dont => Some(Instruction::Dont),
                    ParseInstruction::Mul(a, b) => Some(Instruction::Mul(*a, *b)),
                })
                .collect()
        })
        .expect("parse error")
}

fn part2(instructions: &[Instruction]) -> i32 {
    let mut should_execute = true;
    let mut total = 0;

    for instr in instructions {
        match instr {
            Instruction::Do => should_execute = true,
            Instruction::Dont => should_execute = false,
            Instruction::Mul(a, b) => {
                if should_execute {
                    total += a * b;
                }
            }
        }
    }

    total
}

fn part1(instructions: &[Instruction]) -> i32 {
    instructions
        .iter()
        .filter_map(|instr| {
            if let Instruction::Mul(a, b) = instr {
                Some(a * b)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let matches = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("input").unwrap();
    let input_contents = fs::read_to_string(input_path).expect("Error reading input");
    let instructions = parse_input(&input_contents);

    let result1 = part1(&instructions);
    println!("Part 1: {}", result1);

    let result2 = part2(&instructions);
    println!("Part 2: {}", result2);
}
