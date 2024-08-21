fn main() {
    // Load and parse the input data
    let input: Vec<Vec<u32>> = include_str!("../data/day9.txt")
        .lines()
        .map(|line| line.chars()
                        .map(|c| c.to_digit(10).expect("invalid digit"))
                        .collect())
        .collect();

    println!("{:?}", input);

    let length_vert: usize = input.len();
    let length_horiz: usize = input[0].len();
    let adj_map: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; 

    let mut risk_levels_sum: u32 = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            let mut adj_lower: u8 = 0;
            for offset in adj_map {
                let y: i32 = i as i32 + offset.1;
                let x: i32 = j as i32 + offset.0;
                 
                if is_at_edge(length_vert, length_horiz, i, j) {
                    println!("AT EDGE, i: {}, j: {}", i, j);
                }

                if in_bounds(length_vert, length_horiz, y, x) {
                    if input[y as usize][x as usize] > *num {
                        adj_lower += 1;
                    }
                }
            } 
            
            if is_at_wall(length_vert, length_horiz, i, j) && adj_lower == 3 {
                println!("ADDING, i: {}, j: {}", i, j);
                risk_levels_sum += num + 1;
            } else if is_at_edge(length_vert, length_horiz, i, j) && adj_lower == 2 {
                println!("ADDING, i: {}, j: {}", i, j);
                risk_levels_sum += num + 1;
            } else if adj_lower == 4 {
                risk_levels_sum += num + 1;
            }


            println!("y: {} x: {}, adj_lower: {}, num: {}", i, j, adj_lower, num);
        }
    }

    println!("Risk levels sum: {}", risk_levels_sum);
}

fn in_bounds(length_vert: usize, length_horiz:usize, y: i32, x: i32) -> bool {
     (0 <= x && x < length_horiz as i32) && (0 <= y && y < length_vert as i32)
}

fn is_at_edge(length_vert: usize, length_horiz:usize, i: usize, j: usize) -> bool {
    (i == 0 && j == 0) || (i == 0 && j == length_horiz - 1) || (i == length_vert -1 && j == 0) || (i == length_vert - 1 && j == length_horiz -1) 
}

fn is_at_wall(length_vert: usize, length_horiz:usize, i: usize, j: usize) -> bool {
    (i == 0 && j > 0 && j < length_horiz - 1) 
    || (i > 0 && i < length_vert - 1 && j == 0) 
    || (i == length_vert - 1 && j > 0 && j < length_horiz - 1) 
    || (i > 0 && i < length_vert - 1 && j == length_horiz - 1)
}
