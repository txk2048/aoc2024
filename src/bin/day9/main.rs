use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use clap::{command, value_parser, Arg};

mod part1;
mod part2;

fn parse_disk(input: &str) -> Option<Vec<u8>> {
    let mut disk = Vec::new();

    for c in input.trim().chars() {
        let d = c.to_digit(10)?;
        disk.push(d.try_into().ok()?);
    }

    Some(disk)
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
    let disk = parse_disk(&input_contents).expect("error parsing input");

    let start1 = Instant::now();
    let result1 = part1::part1(&disk);
    let elapsed1 = start1.elapsed();

    println!("Part 1: {}", result1);
    println!("Took {} seconds", elapsed1.as_secs_f64());

    let start2 = Instant::now();
    let result2 = part2::part2(&disk);
    let elapsed2 = start2.elapsed();

    println!("Part 2: {}", result2);
    println!("Took {} seconds", elapsed2.as_secs_f64());
}
