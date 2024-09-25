use std::collections::HashSet;

fn main() {
    let input: Vec<Vec<i32>> = include_str!("../data/day9.txt")
        .lines()
        .map(|line| line.chars()
                        .map(|c| c.to_digit(10).expect("invalid digit") as i32)
                        .collect())
        .collect();

    let mut risk_levels_sum: i32 = 0;

    let low_points = find_low_points(&input);
    
    for idx in low_points {
        risk_levels_sum += input[idx.0][idx.1] + 1;
    }

    println!("Risk levels sum: {}", risk_levels_sum);

    ////////////// Part 2
    let low_points = find_low_points(&input);
    let mut basin_sizes = Vec::new();

    for (x, y) in low_points {
        let size = basin_size(&input, x, y);
        basin_sizes.push(size);
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    let top_3_basin_product: usize = basin_sizes.iter().take(3).product();

    println!("First 3 Basin sizes product: {:?}", top_3_basin_product);
}

fn basin_size(grid: &Vec<Vec<i32>>, x: usize, y: usize) -> usize {
    let mut visited = HashSet::new();
    flood_fill(grid, x, y, &mut visited)
}

fn flood_fill(grid: &Vec<Vec<i32>>, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    if x >= rows || y >= cols || grid[x][y] == 9 || visited.contains(&(x, y)) {
        return 0;
    }

    visited.insert((x, y));

    let mut size = 1;

    if x > 0 && grid[x - 1][y] > grid[x][y] && grid[x - 1][y] != 9 {
        size += flood_fill(grid, x - 1, y, visited);
    }
    if x < rows - 1 && grid[x + 1][y] > grid[x][y] && grid[x + 1][y] != 9 {
        size += flood_fill(grid, x + 1, y, visited);
    }
    if y > 0 && grid[x][y - 1] > grid[x][y] && grid[x][y - 1] != 9 {
        size += flood_fill(grid, x, y - 1, visited);
    }
    if y < cols - 1 && grid[x][y + 1] > grid[x][y] && grid[x][y + 1] != 9 {
        size += flood_fill(grid, x, y + 1, visited);
    }

    size
}

fn find_low_points(grid: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let current = grid[i][j];
            let mut is_low_point = true;

            if i > 0 && grid[i - 1][j] <= current {
                is_low_point = false;
            }
            if i < rows - 1 && grid[i + 1][j] <= current {
                is_low_point = false;
            }
            if j > 0 && grid[i][j - 1] <= current {
                is_low_point = false;
            }
            if j < cols - 1 && grid[i][j + 1] <= current {
                is_low_point = false;
            }

            if is_low_point {
                low_points.push((i, j));
            }
        }
    }

    low_points
}
