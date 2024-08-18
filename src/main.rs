use std::fmt;
use std::vec::Vec;

fn main () {
    let input: Vec<&str> = include_str!("../data/day5.txt").lines().collect();
    println!("{:?}", input);

    let board_size: usize = 1000;
    
    let mut board: Vec<Vec<u16>> = vec![vec![0; board_size]; board_size];

    for item in input {
        let parts: Vec<&str> = item.split("->").map(|x| x.trim()).collect();
        let left: Vec<u16> = parts[0].split(',').map(|x| x.parse().unwrap()).collect();
        let right: Vec<u16> = parts[1].split(',').map(|x| x.parse().unwrap()).collect();

        
        println!("LEFT: {:?}, RIGHT{:?}", left, right);

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
                    board[y as usize][x as usize] += 1; // Modify the value at position (x, y)
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
                    board[y as usize][x as usize] += 1; // Modify the value at position (x, y)
                }
            },
            (false, false) => println!("brother not a line {:?} {:?}", left, right),
            (true, true) => println!("this is just the same thing {:?} {:?}", left, right),
            
        }
    }

    let intersection_count = count_intersections(&board);
    println!("Number of intersections: {}", intersection_count);

    // debug purposes display
    // println!("board: {:?}", board); 
    // let printable: Matrix = Matrix(board.clone()); 
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
                    write!(f, " ")?;
                }
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}


