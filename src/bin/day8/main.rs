use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::{Add, Sub};
use std::path::PathBuf;

use clap::{command, value_parser, Arg};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Vec2> for &Vec2 {
    type Output = <Vec2 as Sub>::Output;

    fn sub(self, rhs: &Vec2) -> Self::Output {
        *self - *rhs
    }
}

#[derive(Clone, Copy)]
struct Dimensions {
    width: i32,
    height: i32,
}

fn parse_input(input: &str) -> Option<(HashMap<char, HashSet<Vec2>>, Dimensions)> {
    let grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let height = grid.len().try_into().ok()?;
    if height == 0 {
        return None;
    }

    let width = grid[0].len().try_into().ok()?;
    if width == 0 {
        return None;
    }

    let dimensions = Dimensions { width, height };
    let mut antennas = HashMap::new();

    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, cell) in row.iter().enumerate() {
            if *cell == '.' {
                continue;
            }

            let x = column_index.try_into().ok()?;
            let y = row_index.try_into().ok()?;

            antennas
                .entry(*cell)
                .or_insert(HashSet::new())
                .insert(Vec2 { x, y });
        }
    }

    Some((antennas, dimensions))
}

fn in_bounds(p: Vec2, dimensions: Dimensions) -> bool {
    0 <= p.x && p.x < dimensions.width && 0 <= p.y && p.y < dimensions.height
}

fn part1(antennas: &HashMap<char, HashSet<Vec2>>, dimensions: Dimensions) -> usize {
    let mut positions = HashSet::new();

    for (_, v) in antennas {
        for a1 in v {
            for a2 in v {
                if a1 == a2 {
                    continue;
                }

                let dy = a2.y - a1.y;
                let dx = a2.x - a1.x;

                positions.insert(Vec2 {
                    x: a1.x - dx,
                    y: a1.y - dy,
                });

                positions.insert(Vec2 {
                    x: a2.x + dx,
                    y: a2.y + dy,
                });
            }
        }
    }

    positions
        .iter()
        .filter(|p| in_bounds(**p, dimensions))
        .count()
}

fn walk_diff(a1: Vec2, a2: Vec2, delta: Vec2, dimensions: Dimensions) -> Vec<Vec2> {
    let mut points = Vec::new();

    let mut p = a1 - delta;
    while in_bounds(p, dimensions) {
        points.push(p);

        p = p - delta;
    }

    p = a2 + delta;
    while in_bounds(p, dimensions) {
        points.push(p);
        p = p + delta;
    }

    points
}

fn part2(antennas: &HashMap<char, HashSet<Vec2>>, dimensions: Dimensions) -> usize {
    let mut positions = HashSet::new();

    for (_, v) in antennas {
        if v.len() > 1 {
            positions.extend(v);
        }

        for a1 in v {
            for a2 in v {
                if a1 == a2 {
                    continue;
                }

                let delta = a2 - a1;
                let pos = walk_diff(*a1, *a2, delta, dimensions);

                positions.extend(pos);
            }
        }
    }

    positions
        .iter()
        .filter(|p| in_bounds(**p, dimensions))
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
    let input_contents = fs::read_to_string(input_path).expect("could not read input");
    let (antennas, dimensions) = parse_input(input_contents.trim()).expect("could not parse input");

    let result1 = part1(&antennas, dimensions);
    println!("Part 1: {}", result1);

    let result2 = part2(&antennas, dimensions);
    println!("Part 2: {}", result2);
}
