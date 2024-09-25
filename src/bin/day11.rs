fn main() {
    let mut input: Vec<Vec<u32>> = include_str!("../data/day11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let vertical: usize = input.len();
    let horizontal: usize = input[0].len();
    
    let mut total_flashes = 0;
    let mut step = 0;
    
    for _ in 1..=300 {
        step += 1;

        let mut flashed: Vec<Vec<bool>> = vec![vec![false; horizontal]; vertical]; 

        input.iter_mut().for_each(|row| row.iter_mut().for_each(|num| *num += 1));

        let mut to_flash = Vec::new();

        for (y, row) in input.iter().enumerate() {
            for (x, &item) in row.iter().enumerate() {
                if item > 9 && !flashed[y][x] {
                    to_flash.push((y, x));
                }
            }
        }

        let mut secondary_flash: Vec<(usize, usize)> = vec![];

        while !to_flash.is_empty() {
            for (y, x) in to_flash.drain(..) {
                if !flashed[y][x] {
                    flashed[y][x] = true;
                    bump_surrounding(&mut input, &mut secondary_flash, y, x);
                }
            }
            to_flash.extend(secondary_flash.drain(..).filter(|&(y, x)| input[y][x] > 9 && !flashed[y][x]));
        }

        to_zero(&mut input, &flashed);
        total_flashes += flashed.iter().flatten().filter(|&&flashed| flashed).count();
        // printeroni(&input);
        
        if step == 100 {
            println!("Total flashes at step 100: {}", total_flashes);
        }

        if flashed.iter().flatten().all(|&flashed| flashed) {
            println!("All octopuses flashed on step: {}", step);
            break;
        }
    }
}

fn bump_surrounding(grid: &mut [Vec<u32>], secondary: &mut Vec<(usize, usize)>, row: usize, col: usize) {
    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];
    
    let rows = grid.len();
    let cols = grid[0].len();
    
    for (dx, dy) in offsets.iter() {
        let new_row = row as i32 + dx;
        let new_col = col as i32 + dy;

        if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
            grid[new_row as usize][new_col as usize] += 1;

            if grid[new_row as usize][new_col as usize] > 9 {
                secondary.push((new_row as usize, new_col as usize));
            }
        }
    }
}

fn to_zero(grid: &mut [Vec<u32>], flashed: &[Vec<bool>]) {
    for (y, row) in flashed.iter().enumerate() {
        for (x, &flashed) in row.iter().enumerate() {
            if flashed {
                grid[y][x] = 0;
            }
        }
    }
}

// fn printeroni(grid: &[Vec<u32>]) {
//     for row in grid {
//         for &item in row {
//             print!("{}", item);
//         }
//         println!();
//     }
//     println!();
// }
