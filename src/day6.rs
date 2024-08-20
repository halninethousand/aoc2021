// smart solution with rotate left, i didn't come up with this.

fn main () {
    let mut input: &str = include_str!("../data/day6.txt");
    let days: u32 = 256;
    let mut age_counts = [0u64; 9];

    // construct counts of each timer
    input
        .split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .for_each(|age| age_counts[age] += 1);

    // run simulation
    for _ in 1..=days {
        age_counts.rotate_left(1); // 0 wraps to 8
        age_counts[6] += age_counts[8]; // 6 increment by prior 0 count
    }

    println!("{}", age_counts.iter().sum::<u64>()); 

}
