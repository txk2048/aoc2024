use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
};

use clap::{command, value_parser, Arg};

type Vec2 = (i32, i32);
type Region = Vec<Vec2>;
type Grid<'a> = &'a [Vec<char>];


fn get_cell(grid: Grid, index: Vec2) -> Option<char> {
    let (x, y) = index;

    let ux: usize = x.try_into().ok()?;
    let uy: usize = y.try_into().ok()?;

    grid.get(uy)?.get(ux).copied()
}

fn cell_not_matches_target(grid: Grid, index: Vec2, target: char) -> bool {
    match get_cell(grid, index) {
        Some(value) => value != target,
        None => true,
    }
}

fn traverse_region(visited: &mut HashSet<Vec2>, grid: Grid, start: Vec2) -> Option<Region> {
    let target = get_cell(grid, start)?;

    let mut region = Vec::new();
    let mut stack = Vec::from([start]);

    while let Some(current) = stack.pop() {
        let (x, y) = current;
        let Some(value) = get_cell(grid, current) else {
            continue;
        };

        if visited.contains(&current) || value != target {
            continue;
        }

        // add to the region
        region.push(current);
        visited.insert(current);

        // iterate neighbors
        let left = (x - 1, y);
        let right = (x + 1, y);
        let up = (x, y - 1);
        let down = (x, y + 1);

        stack.push(left);
        stack.push(right);
        stack.push(up);
        stack.push(down);
    }

    Some(region)
}

fn find_regions(grid: Grid) -> Option<Vec<Region>> {
    let height = grid.len();
    if height == 0 {
        return None;
    }

    let width = grid[0].len();
    if width == 0 {
        return None;
    }

    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut regions = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let p: Vec2 = (x.try_into().unwrap(), y.try_into().unwrap());

            if visited.contains(&p) {
                continue;
            }

            let mut region = traverse_region(&mut visited, grid, p)
                .expect("error occurred while traversing region");

            region.sort();

            regions.push(region);
        }
    }

    // sort region lists for deterministic ordering
    regions.sort();

    Some(regions)
}

fn region_area(region: &Region) -> usize {
    region.len()
}

fn region_perimeter(grid: Grid, region: &Region) -> Option<usize> {
    let mut perimeter = 0;

    for p in region {
        let plant = get_cell(grid, *p)?;

        let (x, y) = *p;

        let north = (x, y - 1);
        let south = (x, y + 1);
        let west = (x - 1, y);
        let east = (x + 1, y);

        if cell_not_matches_target(grid, north, plant) {
            perimeter += 1;
        }

        if cell_not_matches_target(grid, south, plant) {
            perimeter += 1;
        }

        if cell_not_matches_target(grid, west, plant) {
            perimeter += 1;
        }

        if cell_not_matches_target(grid, east, plant) {
            perimeter += 1;
        }
    }

    Some(perimeter)
}

fn region_sides(grid: Grid, region: &Region) -> Option<usize> {
    let mut total_sides = 0;

    let mut north_candidates = HashSet::new();
    let mut south_candidates = HashSet::new();
    let mut west_candidates = HashSet::new();
    let mut east_candidates = HashSet::new();

    for p in region {
        let plant = get_cell(grid, *p)?;

        let (x, y) = *p;

        let north = (x, y - 1);
        let south = (x, y + 1);
        let west = (x - 1, y);
        let east = (x + 1, y);

        let north_west = (x - 1, y - 1);
        let north_east = (x + 1, y - 1);
        let south_west = (x - 1, y + 1);
        let south_east = (x + 1, y + 1);

        if cell_not_matches_target(grid, north, plant) {
            if !north_candidates.contains(&north_west) && !north_candidates.contains(&north_east) {
                total_sides += 1;
            }

            north_candidates.insert(north);
        }

        if cell_not_matches_target(grid, south, plant) {
            if !south_candidates.contains(&south_west) && !south_candidates.contains(&south_east) {
                total_sides += 1;
            }

            south_candidates.insert(south);
        }

        if cell_not_matches_target(grid, west, plant) {
            if !west_candidates.contains(&north_west) && !west_candidates.contains(&south_west) {
                total_sides += 1;
            }
            
            west_candidates.insert(west);
        }

        if cell_not_matches_target(grid, east, plant) {
            if !east_candidates.contains(&north_east) && !east_candidates.contains(&south_east) {
                total_sides += 1;
            }

            east_candidates.insert(east);
        }
    }

    Some(total_sides)
}


fn part1(grid: Grid, regions: &[Region]) -> Option<usize> {
    regions.iter().map(|region|  {
        let perimeter = region_perimeter(grid, region)?;
        let area = region_area(region);

        Some(perimeter * area)
    }).sum()
}

fn part2(grid: Grid, regions: &[Region]) -> Option<usize> {
    regions.iter().map(|region|  {
        let sides = region_sides(grid, region)?;
        let area = region_area(region);

        Some(sides * area)
    }).sum()
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

    let input_data = fs::read_to_string(input_path).expect("Could not open input file");
    let grid: Vec<_> = input_data
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let regions = find_regions(&grid).expect("could not find regions");

    let result1 = part1(&grid, &regions).expect("could not solve part 1");
    println!("Part 1: {}", result1);


    let result2 = part2(&grid, &regions).expect("could not solve part 2");
    println!("Part 2: {}", result2);
}

#[cfg(test)]
mod tests {
    use crate::find_regions;

    #[rustfmt::skip::macros(vec)]
    #[test]
    fn test_find_regions() {
        let data = vec![
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
        ];

        let mut expected_regions = vec![
            vec![
                (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), 
                (0, 1),         (2, 1),         (4, 1),
                (0, 2), (1, 2), (2, 2), (3, 2), (4, 2),
                (0, 3),         (2, 3),         (4, 3),
                (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
            ],
            vec![(1, 1)],
            vec![(1, 3)],
            vec![(3, 1)],
            vec![(3, 3)],
        ];

        expected_regions.iter_mut().for_each(|region| region.sort());
        expected_regions.sort();

        let regions = find_regions(&data).expect("error finding regions");
        assert_eq!(regions, expected_regions);
    }
}
