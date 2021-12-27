use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> std::io::Result<()> {

    let path = env::current_dir()?;
    println!("{}", path.display());

    let file = File::open("../data/input.txt")?;
    let reader = BufReader::new(file);
    let mut depths = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        depths.push(line.parse::<i32>().unwrap());
    }

    // println!("{:?}", depths);

    let mut measurements = Vec::new();
    let mut c: i32 = 0;
    for idx in 0..depths.len() {
        if idx == 0 || idx == depths.len()-1 {
            continue;
        }
        measurements.push(&depths[idx-1] + &depths[idx] + &depths[idx+1]);
    }
    for idx in 0..measurements.len() {
        if idx == 0 {
            continue;
        }
        if &measurements[idx] > &measurements[idx-1] {
            c += 1;
        }
        println!{"{}", c};
    }
    Ok(())

}