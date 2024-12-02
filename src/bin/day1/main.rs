use clap::{arg, command};
use std::collections::HashMap;
use std::fs;
use std::num::ParseIntError;

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let nums: Vec<_> = line.split(' ').collect();
        let num1: i32 = nums.first().expect("could not get first").parse()?;
        let num2: i32 = nums.last().expect("could not get last").parse()?;

        list1.push(num1);
        list2.push(num2);
    }

    list1.sort();
    list2.sort();

    Ok((list1, list2))
}

fn part1(list1: &[i32], list2: &[i32]) -> i32 {
    let mut diff = 0;

    for (a, b) in list1.iter().zip(list2) {
        diff += (a - b).abs();
    }

    diff
}

fn part2(list1: &[i32], list2: &[i32]) -> i32 {
    let mut count_right = HashMap::new();
    for item in list2 {
        if let Some(count) = count_right.get_mut(item) {
            *count += 1;
        } else {
            count_right.insert(item, 1);
        }
    }

    let mut total = 0;
    for item in list1 {
        total += item * count_right.get(item).copied().unwrap_or(0);
    }

    total
}

fn main() {
    let matches = command!().arg(arg!(<input> "Input filename")).get_matches();

    let input_filename = matches
        .get_one::<String>("input")
        .expect("input file required");
    let input_contents = fs::read_to_string(input_filename).expect("could not read input file");
    let (list1, list2) = parse_input(&input_contents).expect("could not parse input");

    let result1 = part1(&list1, &list2);
    println!("Part 1: {}", result1);

    let result2 = part2(&list1, &list2);
    println!("Part 2: {}", result2);
}
