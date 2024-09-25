use std::fmt;
use std::vec::Vec;

fn main () {
    let input: Vec<&str> = include_str!("../data/day5.txt").lines().collect();

    let board_size: usize = 1000;
    
    let mut board: Vec<Vec<u16>> = vec![vec![0; board_size]; board_size];
    let mut board_2 = board.clone();

    for item in input {
        let parts: Vec<&str> = item.split("->").map(|x| x.trim()).collect();
        let left: Vec<u16> = parts[0].split(',').map(|x| x.parse().unwrap()).collect();
        let right: Vec<u16> = parts[1].split(',').map(|x| x.parse().unwrap()).collect();

        match is_a_line(&left, &right) {
            (true, false) => { // horizontal line
                let mut start_y: u16 = left[1];      
                let mut destination_y: u16 = right[1]; 
                let x: u16 = left[0];      

                if start_y > destination_y {
                    std::mem::swap(&mut start_y, &mut destination_y);
                }

                let range = start_y..=destination_y;

                for y in range {
                    board[y as usize][x as usize] += 1;
                    board_2[y as usize][x as usize] += 1;
                }
            },
            (false, true) => { // vertical line
                let mut start_x: u16 = left[0];      
                let mut destination_x: u16 = right[0]; 
                let y: u16 = left[1];      

                if start_x > destination_x {
                    std::mem::swap(&mut start_x, &mut destination_x);
                }

                let range = start_x..=destination_x;

                for x in range {
                    board[y as usize][x as usize] += 1;
                    board_2[y as usize][x as usize] += 1;
                }
            },
            (false, false) => {
                /*
                (5 , 5) -> (2, 2) : UpLeft      (x and y increase)
                (5 , 5) -> (8, 2) : UpRight     (x increases, y decreases)
                (5 , 5) -> (2, 8) : DownLeft    (x decreases, y increases)
                (5 , 5) -> (8, 8) : DownRight   (x and y increase)
                */
                let mut start_x = left[0];
                let mut start_y = left[1];
                let destination_x = right[0];
                let destination_y = right[1];
                let dx = destination_x as i16 - start_x as i16;
                let dy = destination_y as i16 - start_y as i16;

                match (dx, dy) {
                    (dx, dy) if dx == dy && dx < 0 => {     // Both x and y decrease
                        for _ in 0..=(start_x - destination_x) {
                            board_2[start_y as usize][start_x as usize] += 1; // Modify the value at position (x, y)
                            start_x -= 1; 
                            start_y -= 1; 
                        }
                    },
                    (dx, dy) if dx == -dy && dx > 0 => {    // x increases, y decreases
                        for _ in 0..=(start_y - destination_y) {
                            board_2[start_y as usize][start_x as usize] += 1; // Modify the value at position (x, y)
                            start_x += 1; 
                            start_y -= 1; 
                        }
                    },
                    (dx, dy) if dx == -dy && dx < 0 => {    // x decreases, y increases
                        for _ in 0..=(start_x - destination_x) { 
                            board_2[start_y as usize][start_x as usize] += 1; // Modify the value at position (x, y)
                            start_x -= 1; 
                            start_y += 1; 
                        }
                    },
                    (dx, dy) if dx == dy && dx > 0 => {     // Both x and y increase
                        for _ in 0..=(destination_x - start_x) {
                            board_2[start_y as usize][start_x as usize] += 1; // Modify the value at position (x, y)
                            start_x += 1; 
                            start_y += 1; 
                        }
                    },
                    _ => println!("not diagonal"), 
                };
            },
            (true, true) => println!("this is just the same thing {:?} {:?}", left, right),
            
        }
    }

    let intersection_count = count_intersections(&board);
    let intersection_count_2 = count_intersections(&board_2);
    println!("Number of intersections only lines: {}", intersection_count);
    println!("Number of intersections lines and diagonals: {}", intersection_count_2);

    // debug purposes display
    // let printable: Matrix = Matrix(board_2.clone()); 
    // println!("{}", printable);
}

fn is_a_line(left: &[u16], right: &[u16]) -> (bool, bool) {
    let x: bool = left[0] == right[0];
    let y: bool = left[1] == right[1];

    (x, y)
}

fn count_intersections(board: &Vec<Vec<u16>>) -> usize {
    board.iter()
        .flatten()
        .filter(|&&cell| cell > 1)
        .count()
}

struct Matrix(Vec<Vec<u16>>);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for (i, &cell) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, "")?;
                }
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


