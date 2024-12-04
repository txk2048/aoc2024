use std::fs;
use std::path::PathBuf;

use clap::{command, value_parser, Arg};

mod part1;

fn is_mas(arr: [char; 3]) -> bool {
    (arr[0] == 'M' && arr[1] == 'A' && arr[2] == 'S')
        || (arr[0] == 'S' && arr[1] == 'A' && arr[2] == 'M')
}

fn part2(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    if height == 0 {
        return 0;
    }

    let width = grid[0].len();

    let mut total = 0;
    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            let diagonal1 = [
                grid[row - 1][col - 1],
                grid[row][col],
                grid[row + 1][col + 1],
            ];
            let diagonal2 = [
                grid[row - 1][col + 1],
                grid[row][col],
                grid[row + 1][col - 1],
            ];

            if is_mas(diagonal1) && is_mas(diagonal2) {
                total += 1;
            }
        }
    }

    total
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
    let input_contents = fs::read_to_string(input_path).expect("could not read input");
    let trimmed_input: Vec<_> = input_contents
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let result1 = part1::part1(&trimmed_input);
    println!("Part 1: {}", result1);

    let result2 = part2(&trimmed_input);
    println!("Part 2: {}", result2);
}
