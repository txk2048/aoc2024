use clap::{command, value_parser, Arg};
use std::{fs, path::PathBuf};

mod parser;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vec2 {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
struct Machine {
    pub button_a: Vec2,
    pub button_b: Vec2,
    pub prize: Vec2,
}

fn solve(machine: &Machine) -> Option<f64> {
    // following variables represent a 3x2 matrix
    // a b c
    // d e f

    let a = machine.button_a.x as f64;
    let b = machine.button_b.x as f64;
    let c = machine.prize.x as f64;
    let mut d = machine.button_a.y as f64;
    let mut e = machine.button_b.y as f64;
    let mut f = machine.prize.y as f64;

    let orig_d = d;
    d *= a;
    e *= a;
    f *= a;

    d -= a * orig_d;
    e -= b * orig_d;
    f -= c * orig_d;

    // d should now be 0
    assert_eq!(d, 0.0);

    // solve bottom row
    let y = f / e;

    // solve top row using y
    let x = (c - b * y) / a;

    if x.fract() == 0.0 && y.fract() == 0.0 {
        return Some(3.0 * x + y);
    }

    None
}

fn part1(machines: &[Machine]) -> f64 {
    machines.iter().filter_map(|m| solve(m)).sum()
}

fn part2(machines: &[Machine]) -> f64 {
    machines
        .iter()
        .map(|machine| {
            let new_prize = Vec2 {
                x: machine.prize.x + 10000000000000,
                y: machine.prize.y + 10000000000000,
            };

            Machine {
                button_a: machine.button_a,
                button_b: machine.button_b,
                prize: new_prize,
            }
        })
        .filter_map(|m| solve(&m))
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

    let input_path = args
        .get_one::<PathBuf>("input")
        .expect("Could not get input filename");

    let input_contents = fs::read_to_string(input_path).expect("Could not read input file");
    let machines = parser::parse_machines(input_contents.trim()).expect("Could not parse input");

    let result1 = part1(&machines);
    println!("Part 1: {}", result1);

    let result2 = part2(&machines);
    println!("Part 2: {}", result2);
}
