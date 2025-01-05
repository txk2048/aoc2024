use clap::{command, value_parser, Arg};

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn parse_stones(input: &str) -> Option<Vec<u64>> {
    let stones = input
        .split(' ')
        .map(|x| x.parse::<u64>().ok())
        .collect::<Option<_>>();

    stones
}

fn blink_stone(stone: u64, count: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    // no more expansions, so only one element
    if count == 0 {
        return 1;
    }

    // we have seen this call before
    // return cached value
    if let Some(v) = memo.get(&(stone, count)) {
        return *v;
    }

    let result = if stone == 0 {
        blink_stone(1, count - 1, memo)
    } else if stone.to_string().len() % 2 == 0 {
        let s = stone.to_string();
        let half = s.len() / 2;

        let left = (s[..half]).parse().unwrap();
        let right = (s[half..]).parse().unwrap();

        blink_stone(left, count - 1, memo) + blink_stone(right, count - 1, memo)
    } else {
        blink_stone(stone * 2024, count - 1, memo)
    };

    memo.insert((stone, count), result);
    return result;
}

fn blink(stones: &[u64], count: u64) -> u64 {
    let mut memo = HashMap::new();

    stones
        .iter()
        .map(|stone| blink_stone(*stone, count, &mut memo))
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

    let input_file_path = args.get_one::<PathBuf>("input").unwrap();
    let input_file_contents =
        fs::read_to_string(input_file_path).expect("could not read input file");

    let stones = parse_stones(input_file_contents.trim()).expect("could not parse input");

    let result1 = blink(&stones, 25);
    println!("Part 1: {}", result1);

    let result2 = blink(&stones, 75);
    println!("Part 2: {}", result2);
}
