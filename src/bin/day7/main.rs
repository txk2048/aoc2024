use std::fs;
use std::path::PathBuf;

use clap::{command, value_parser, Arg};

struct Equation {
    result: u64,
    terms: Vec<u64>,
}

fn parse_input(input: &str) -> Result<Vec<Equation>, String> {
    input
        .lines()
        .map(|line| {
            let (res, terms) = line
                .split_once(": ")
                .ok_or(String::from("invalid input format: missing colon"))?;

            let result = res
                .parse()
                .map_err(|e| format!("could not parse result: {}", e))?;

            let terms = terms
                .split(' ')
                .map(|x| {
                    x.parse()
                        .map_err(|e| format!("could not parse term: {}", e))
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Equation { result, terms })
        })
        .collect()
}

fn concat_num(current: u64, next: u64) -> u64 {
    format!("{}{}", current, next).parse().unwrap()
}

fn is_solvable(equation: &Equation, enable_concat_operator: bool) -> bool {
    let mut stack = Vec::new();

    if let Some((first, rem)) = equation.terms.split_first() {
        stack.push((*first, rem));
    }

    while let Some((current, rem)) = stack.pop() {
        if rem.len() == 0 && current == equation.result {
            return true;
        }

        if let Some((next, r)) = rem.split_first() {
            if let Some(n) = current.checked_add(*next) {
                stack.push((n, r));
            }

            if let Some(n) = current.checked_mul(*next) {
                stack.push((n, r));
            }

            if enable_concat_operator {
                let c = concat_num(current, *next);
                stack.push((c, r));
            }
        }
    }

    false
}

fn part1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| is_solvable(e, false))
        .map(|e| e.result)
        .sum()
}

fn part2(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| is_solvable(e, true))
        .map(|e| e.result)
        .sum()
}

fn main() {
    let args = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input_path = args.get_one::<PathBuf>("input").unwrap();
    let input_contents = fs::read_to_string(input_path).expect("could not read input");
    let equations = parse_input(input_contents.trim()).expect("could not parse input");

    let result1 = part1(&equations);
    println!("Part 1: {}", result1);

    let result2 = part2(&equations);
    println!("Part 2: {}", result2);
}
