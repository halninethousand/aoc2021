use std::collections::HashSet;

fn main() {
    // Load and parse the input data
    let input: Vec<i32> = include_str!("../data/current.txt")
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

        println!("Fuel {} for {}", total_fuel, target_position);
    }

    println!("Minimum fuel required, increasing fuel cost: {}", minimum_fuel);
}
