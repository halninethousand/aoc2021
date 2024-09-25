use std::collections::HashSet;

fn main() {
    let input: Vec<i32> = include_str!("../../data/day7.txt")
        .split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect();

    let unique_positions: HashSet<_> = input.iter().cloned().collect();

    let mut minimum_fuel = i32::MAX;

    for target_position in unique_positions.clone() {
        let total_fuel: i32 = input.iter()
            .map(|&pos| (pos - target_position).abs())
            .sum();

        if total_fuel < minimum_fuel {
            minimum_fuel = total_fuel;
        }
    }

    println!("Minimum fuel required: {}", minimum_fuel);

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let unique_positions: HashSet<_> = (min..=max).collect();

    let mut minimum_fuel = i32::MAX;

    let fuel_cost = |distance: i32| -> i32 {
        (distance * (distance + 1)) / 2
    };

    for target_position in unique_positions {
        let total_fuel: i32 = input.iter()
            .map(|&pos| fuel_cost((pos - target_position).abs()))
            .sum();
        if total_fuel < minimum_fuel {
            minimum_fuel = total_fuel;
        }
    }

    println!("Minimum fuel required, increasing fuel cost: {}", minimum_fuel);
}
