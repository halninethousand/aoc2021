use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    let path = env::current_dir()?;
    println!("{}", path.display());


    let file = File::open("../data/input_3.txt")?;
    let reader = BufReader::new(file);

    let mut positions: HashMap<usize, isize> = HashMap::new();

    let mut bits = reader.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut bits_2 = bits.clone();

    sift(&bits, &mut positions);

    println!("{}", bits.len());
    println!("{:?}", positions);

    let mut checker = "".to_string();
    for num in 0..12 {
        if positions.get(&num).unwrap() < &0 {
            if bits.len() == 1{
                break
            }
            checker.push('0');
            bits.retain(|x| x.starts_with(&checker));
            println!("{:?}", bits);
            positions.drain();
            sift(&bits, &mut positions);
            println!("{:?}", positions);
        } else {
            if bits.len() == 1{
                break
            }
            checker.push('1');
            bits.retain(|x| x.starts_with(&checker));
            println!("{:?}", bits);
            positions.drain();
            sift(&bits, &mut positions);
            println!("{:?}", positions);
        }
    }

    let mut checker = "".to_string();
    for num in 0..12 {
        if positions.get(&num).unwrap() < &0 {
            if bits_2.len() == 1{
                break
            }
            checker.push('1');
            bits_2.retain(|x| x.starts_with(&checker));
            println!("{:?}", bits_2);
            positions.drain();
            sift(&bits_2, &mut positions);
        } else {
            if bits_2.len() == 1{
                break
            }
            checker.push('0');
            bits_2.retain(|x| x.starts_with(&checker));
            println!("{:?}", bits_2);
            positions.drain();
            sift(&bits_2, &mut positions);
        }
    }
    println!("{:?}", bits);
    println!("{:?}", bits_2);
    Ok(())
    

}

fn sift(bits: &Vec<String>, positions: &mut HashMap<usize, isize>) {
    for item in bits {
        for (idx, c) in item.chars().enumerate() {
            let count = positions.entry(idx).or_insert(0);
            match c {
                '0' => *count -= 1,
                '1' => *count += 1,
                _ => println!("unknown bit")
            }
        }
    }
}
