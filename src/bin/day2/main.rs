use clap::{command, value_parser, Arg};
use std::fs;
use std::path::PathBuf;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let report = line
            .split(' ')
            .map(|level| level.parse::<i32>().expect("Could not parse level"))
            .collect();

        reports.push(report);
    }

    reports
}

fn is_safe(report: &[i32]) -> bool {
    let mut prev_diff = None;

    for items in report.windows(2) {
        let [a, b] = items else {
            panic!("items did not have 2 items")
        };
        let diff = b - a;
        let abs_diff = diff.abs();

        if abs_diff < 1 || abs_diff > 3 {
            return false;
        }

        if let Some(d) = prev_diff {
            // if their sign differs
            if (d < 0 && diff > 0) || (d > 0 && diff < 0) {
                return false;
            }
        }

        prev_diff = Some(diff);
    }

    true
}

fn part1(reports: &[Vec<i32>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

fn part2(reports: &[Vec<i32>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            if is_safe(report) {
                return true;
            }

            report
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let mut new_report = (*report).clone();
                    new_report.remove(index);

                    return new_report;
                })
                .any(|report| is_safe(&report))
        })
        .count()
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
    let input_contents = fs::read_to_string(input_path).expect("Could not read input");
    let reports = parse_input(input_contents.trim());

    let result1 = part1(&reports);
    println!("Part 1: {}", result1);

    let result2 = part2(&reports);
    println!("Part 2: {}", result2);
}
