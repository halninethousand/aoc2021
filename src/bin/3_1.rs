use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    let path = env::current_dir()?;
    println!("{}", path.display());


    let file = File::open("../data/input_3.txt")?;
    let reader = BufReader::new(file);
    let mut bits = Vec::new();

    let mut positions = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        bits.push(line);
    }

    for item in &bits {
        for (idx, c) in item.chars().enumerate() {
            let count = positions.entry(idx).or_insert(0);
            match c {
                '0' => *count -= 1,
                '1' => *count += 1,
                _ => println!("unknown bit")
            }
        }
    }
    
    println!("{:?}", positions);
    let mut result = Vec::new();
    for idx in 0..12 {
        result.insert(idx, 0);
    }

    for (pos, val) in &positions {
        result[*pos] = *val;
    }
    
    println!("{:?}",result);
    Ok(())
    

}