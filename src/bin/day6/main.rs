use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use clap::{command, value_parser, Arg};
use rayon::prelude::*;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Dimensions {
    width: i32,
    height: i32,
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn step(point: Point, direction: Direction) -> Point {
    match direction {
        Direction::Up => Point {
            x: point.x,
            y: point.y - 1,
        },
        Direction::Down => Point {
            x: point.x,
            y: point.y + 1,
        },
        Direction::Left => Point {
            x: point.x - 1,
            y: point.y,
        },
        Direction::Right => Point {
            x: point.x + 1,
            y: point.y,
        },
    }
}

fn parse_input(input: &str) -> Option<(Point, HashSet<Point>, Dimensions)> {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let dimensions = {
        let height = grid.len().try_into().ok()?;
        if height == 0 {
            return None;
        }

        let width = grid[0].len().try_into().ok()?;
        if width == 0 {
            return None;
        }

        Dimensions { width, height }
    };

    let mut guard = None;
    let mut obstacles = HashSet::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            let x = column_index.try_into().ok()?;
            let y = row_index.try_into().ok()?;
            let p = Point { x, y };

            if *column == '^' {
                guard = Some(p);
            } else if *column == '#' {
                obstacles.insert(p);
            }
        }
    }

    guard.map(|g| (g, obstacles, dimensions))
}

fn run_path(
    mut guard: Point,
    obstacles: &HashSet<Point>,
    dimensions: Dimensions,
) -> HashSet<Point> {
    let mut visited = HashSet::new();
    let mut heading = Direction::Up;

    while (0 <= guard.x && guard.x < dimensions.width)
        && (0 <= guard.y && guard.y < dimensions.height)
    {
        visited.insert(guard);

        let next = step(guard, heading);

        if obstacles.contains(&next) {
            heading = turn_right(heading);
        } else {
            guard = next;
        }
    }

    visited
}

fn does_guard_loop(
    mut guard: Point,
    obstacles: &HashSet<Point>,
    dimensions: Dimensions,
    extra_obstacle: Point,
) -> bool {
    let mut visited = HashSet::new();
    let mut heading = Direction::Up;

    while (0 <= guard.x && guard.x < dimensions.width)
        && (0 <= guard.y && guard.y < dimensions.height)
    {
        let state = (guard, heading);

        if visited.contains(&state) {
            return true;
        }

        visited.insert(state);

        let next = step(guard, heading);

        if next == extra_obstacle || obstacles.contains(&next) {
            heading = turn_right(heading);
        } else {
            guard = next;
        }
    }

    false
}

fn part1(guard: Point, obstacles: &HashSet<Point>, dimensions: Dimensions) -> usize {
    let walk = run_path(guard, &obstacles, dimensions);
    walk.len()
}

fn part2(guard: Point, obstacles: &HashSet<Point>, dimensions: Dimensions) -> usize {
    let visited_points = run_path(guard, obstacles, dimensions);

    visited_points
        .par_iter()
        .filter(|p| **p != guard)
        .filter(|p| does_guard_loop(guard, obstacles, dimensions, **p))
        .count()
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
    let input_contents = fs::read_to_string(input_path).expect("could not read input file");
    let (guard, obstacles, dimensions) = parse_input(&input_contents).expect("invalid input");

    let result1 = part1(guard, &obstacles, dimensions);
    println!("Part 1: {}", result1);

    let result2 = part2(guard, &obstacles, dimensions);
    println!("Part 2: {}", result2);
}
