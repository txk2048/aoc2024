use clap::{command, Arg, value_parser};
use std::path::PathBuf;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line.split(' ')
                .map(|level| level.parse::<i32>().expect("Could not parse level"))
                .collect();

        reports.push(report);
    }

    reports
}

fn is_safe(report: &[i32]) -> bool {
    let mut is_increasing = false;
    let mut is_decreasing = false;

    for (a, b) in report.iter().zip(report.iter().skip(1)) {
        let diff = b - a;
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        if diff > 0 {
            is_increasing = true;
        } else if diff < 0 {
            is_decreasing = true;
        }
    }

    is_increasing ^ is_decreasing
}

fn part1(reports: &[Vec<i32>]) -> usize {
    reports.iter()
           .filter(|report| is_safe(report))
           .count()
}

fn main() {
    let matches = command!()
        .arg(
            Arg::new("input")
                .required(true)
                .value_parser(value_parser!(PathBuf))
        )
        .get_matches();

    let input_path = matches.get_one::<PathBuf>("input").unwrap();
    let input_contents = fs::read_to_string(input_path).expect("Could not read input");
    let reports = parse_input(input_contents.trim());

    let result1 = part1(&reports);
    println!("Part 1: {}", result1);
}
