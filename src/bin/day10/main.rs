use clap::{command, value_parser, Arg};

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

fn find_trailheads(grid: &[Vec<u8>]) -> Vec<Vec2> {
    let mut trailheads = Vec::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if *column == 0 {
                trailheads.push(Vec2 {
                    x: column_index,
                    y: row_index,
                });
            }
        }
    }

    trailheads
}

// use DFS to find peaks
fn find_unique_peaks(grid: &[Vec<u8>], trailhead: Vec2) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut peaks: HashSet<Vec2> = HashSet::new();

    let mut stack = Vec::new();
    stack.push(trailhead);

    while let Some(p) = stack.pop() {
        let current = grid[p.y][p.x];

        // we have found a peak
        if current == 9 {
            peaks.insert(p);
            continue;
        }

        // add neighbors if they are viable

        // up
        if p.y > 0 && grid[p.y - 1][p.x] == current + 1 {
            stack.push(Vec2 { x: p.x, y: p.y - 1 });
        }

        // down
        if p.y + 1 < height && grid[p.y + 1][p.x] == current + 1 {
            stack.push(Vec2 { x: p.x, y: p.y + 1 });
        }

        // left
        if p.x > 0 && grid[p.y][p.x - 1] == current + 1 {
            stack.push(Vec2 { x: p.x - 1, y: p.y });
        }

        // right
        if p.x + 1 < width && grid[p.y][p.x + 1] == current + 1 {
            stack.push(Vec2 { x: p.x + 1, y: p.y });
        }
    }

    peaks.len()
}

// use DFS to find peaks
fn find_paths_to_peaks(grid: &[Vec<u8>], trailhead: Vec2) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut encountered_peaks = 0;

    let mut stack = Vec::new();
    stack.push(trailhead);

    while let Some(p) = stack.pop() {
        let current = grid[p.y][p.x];

        // we have found a peak
        if current == 9 {
            encountered_peaks += 1;
            continue;
        }

        // add neighbors if they are viable

        // up
        if p.y > 0 && grid[p.y - 1][p.x] == current + 1 {
            stack.push(Vec2 { x: p.x, y: p.y - 1 });
        }

        // down
        if p.y + 1 < height && grid[p.y + 1][p.x] == current + 1 {
            stack.push(Vec2 { x: p.x, y: p.y + 1 });
        }

        // left
        if p.x > 0 && grid[p.y][p.x - 1] == current + 1 {
            stack.push(Vec2 { x: p.x - 1, y: p.y });
        }

        // right
        if p.x + 1 < width && grid[p.y][p.x + 1] == current + 1 {
            stack.push(Vec2 { x: p.x + 1, y: p.y });
        }
    }

    encountered_peaks
}

fn part1(grid: &[Vec<u8>]) -> usize {
    let trailheads = find_trailheads(grid);

    trailheads
        .iter()
        .map(|trailhead| find_unique_peaks(grid, *trailhead))
        .sum()
}

fn part2(grid: &[Vec<u8>]) -> usize {
    let trailheads = find_trailheads(grid);

    trailheads
        .iter()
        .map(|trailhead| find_paths_to_peaks(grid, *trailhead))
        .sum()
}

fn parse_grid(input: &str) -> Option<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).map(|x| x as u8))
                .collect::<Option<_>>()
        })
        .collect::<Option<_>>()
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
    let input = fs::read_to_string(input_file_path).expect("could not read input file");
    let grid = parse_grid(input.trim()).expect("could not parse input");

    let result1 = part1(&grid);
    println!("Part 1: {}", result1);

    let result2 = part2(&grid);
    println!("Part 2: {}", result2);
}
