use std::collections::HashMap;

fn main() {
    let mut input: Vec<&str> = include_str!("../../data/day14.txt").lines().collect();
    let mut polymer: Vec<char> = input[0].chars().collect();
    let polymer_2 = polymer.clone();
    input.drain(..2);
    let map = build_map(input);
    let mut polymer_steps = 10;

    println!("{map:?}");
    println!("polymer: {polymer:?}");
    

    // Part1
    for _ in 0..polymer_steps {
        let mut insert_pointer = 1;

        let pairs: Vec<String> = polymer
            .windows(2)
            .map(|window| format!("{}{}", window[0], window[1]))
            .collect();

        for pair in pairs {
            if let Some(x) = map.get(&pair) {
                polymer.insert(insert_pointer, *x);
                insert_pointer += 2;
            } else {
                insert_pointer += 1;
            }
        }
    }

    let char_counts = count_char_occurrences(polymer.clone());
    let (min_char, max_char) = find_min_max_char(&char_counts);

    let min_ch = min_char.map(|(ch, count)| (*ch, *count));
    let max_ch = max_char.map(|(ch, count)| (*ch, *count));

    if let Some((_, count_min)) = min_ch {
        if let Some((_, count_max)) = max_ch {
            println!("Part1: {}", count_max - count_min);
        }
    }

    
    // Part2 we can no longer simulate the string so we count the newly produced pairs
    polymer_steps = 40;
    let mut pair_counts = HashMap::new();
    for window in polymer_2.windows(2) {
        let pair = format!("{}{}", window[0], window[1]);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    let mut char_counts = count_char_occurrences(polymer_2.clone());

    for _ in 0..polymer_steps {
        let mut new_pair_counts = HashMap::new();

        for (pair, count) in pair_counts {
            if let Some(&insert_char) = map.get(&pair) {
                let chars: Vec<char> = pair.chars().collect();
                let new_pair1 = format!("{}{}", chars[0], insert_char);
                let new_pair2 = format!("{}{}", insert_char, chars[1]);

                *new_pair_counts.entry(new_pair1).or_insert(0) += count;
                *new_pair_counts.entry(new_pair2).or_insert(0) += count;

                *char_counts.entry(insert_char).or_insert(0) += count;
            } else {
                *new_pair_counts.entry(pair).or_insert(0) += count;
            }
        }

        pair_counts = new_pair_counts;
    }

    let (min_char, max_char) = find_min_max_char(&char_counts);

    let min_ch = min_char.map(|(_, count)| *count);
    let max_ch = max_char.map(|(_, count)| *count);

    if let (Some(count_min), Some(count_max)) = (min_ch, max_ch) {
        println!("Part2: {}", count_max - count_min);
    }
}

fn build_map(pairs: Vec<&str>) -> HashMap<String, char> {
    pairs.into_iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(" -> ").collect();
            (parts[0].to_string(), parts[1].chars().next().unwrap())
        })
        .collect()
}

fn count_char_occurrences(chars: Vec<char>) -> HashMap<char, usize> {
    chars.into_iter().fold(HashMap::new(), |mut counts, ch| {
        *counts.entry(ch).or_insert(0) += 1;
        counts
    })
}

fn find_min_max_char(counts: &HashMap<char, usize>) -> (Option<(&char, &usize)>, Option<(&char, &usize)>) {
    let min_char = counts.iter().min_by_key(|entry| entry.1);
    let max_char = counts.iter().max_by_key(|entry| entry.1);
    (min_char, max_char)
}
