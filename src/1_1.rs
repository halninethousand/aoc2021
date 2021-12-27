use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() -> std::io::Result<()> {

    let path = env::current_dir()?;
    println!("{}", path.display());

    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let mut depths = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
        // Show the line and its number.
        println!("{}. {}", index + 1, line);
        depths.push(line.parse::<i32>().unwrap());
    }
    println!("{:?}", depths);
    for depth in depths.iter() {
        println!("{}", depth)
    }
    let mut c: i32 = 0;
    for idx in 0..depths.len() {
        if idx == 0 {
            continue;
        }
        if &depths[idx] > &depths[idx-1] {
            c += 1;
            println!("bigger")
        }
        println!{"{}", c};
    }
    Ok(())

}