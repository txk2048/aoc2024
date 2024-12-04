pub(crate) fn part1(grid: &[Vec<char>]) -> usize {
    let height = grid.len();

    if height == 0 {
        return 0;
    }

    let width = grid[0].len();
    let directions = [
        left, right, up, down, up_left, up_right, down_left, down_right,
    ];

    let mut total = 0;
    for row in 0..height {
        for col in 0..width {
            total += directions.iter().filter(|f| f(grid, row, col)).count();
        }
    }

    total
}

fn left(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    if col < 3 {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row][col - 1] == 'M'
            && grid[row][col - 2] == 'A'
            && grid[row][col - 3] == 'S'
    }
}

fn right(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let width = grid[0].len();

    if col + 3 >= width {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row][col + 1] == 'M'
            && grid[row][col + 2] == 'A'
            && grid[row][col + 3] == 'S'
    }
}

fn up(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    if row < 3 {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row - 1][col] == 'M'
            && grid[row - 2][col] == 'A'
            && grid[row - 3][col] == 'S'
    }
}

fn down(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let height = grid.len();

    if row + 3 >= height {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row + 1][col] == 'M'
            && grid[row + 2][col] == 'A'
            && grid[row + 3][col] == 'S'
    }
}

fn up_left(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    if row < 3 || col < 3 {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row - 1][col - 1] == 'M'
            && grid[row - 2][col - 2] == 'A'
            && grid[row - 3][col - 3] == 'S'
    }
}

fn up_right(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let width = grid[0].len();

    if row < 3 || col + 3 >= width {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row - 1][col + 1] == 'M'
            && grid[row - 2][col + 2] == 'A'
            && grid[row - 3][col + 3] == 'S'
    }
}

fn down_left(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let height = grid.len();

    if row + 3 >= height || col < 3 {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row + 1][col - 1] == 'M'
            && grid[row + 2][col - 2] == 'A'
            && grid[row + 3][col - 3] == 'S'
    }
}

fn down_right(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    if row + 3 >= height || col + 3 >= width {
        false
    } else {
        grid[row][col] == 'X'
            && grid[row + 1][col + 1] == 'M'
            && grid[row + 2][col + 2] == 'A'
            && grid[row + 3][col + 3] == 'S'
    }
}
