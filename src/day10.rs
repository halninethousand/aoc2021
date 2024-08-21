fn main() {
    let input: Vec<&str> = include_str!("../data/day10.txt").lines().collect();
    
    let openers: [char; 4] = ['[', '<', '(', '{'];
    let mut score: u32 = 0;
    let mut completion_scores: Vec<u64> = vec![];

    'line: for line in input {
        // if there are unclosed brackets they will be left in open at the end
        let mut open: Vec<char> = vec![];

        for ch in line.chars() {
            if openers.contains(&ch) {
                open.push(ch);
            } else if let Some(last_opened) = open.iter().last() {
                match ch {
                    ')' if *last_opened == '(' => {
                        open.pop();
                    },

                    ']' if *last_opened == '[' => {
                        open.pop();
                    },

                    '}' if *last_opened == '{' => {
                        open.pop();
                    },

                    '>' if *last_opened == '<' => {
                        open.pop();
                    },

                    x => {
                        match x {
                            ')' => score += 3,
                            ']' => score += 57,
                            '}' => score += 1197,
                            '>' => score += 25137,
                            _ => unreachable!("nice input guy"),
                        };
                        continue 'line;
                    },
                };
            }
        }

        let mut end_score: u64 = 0; 
        
        open.reverse();

        for item in &open {
            match item {
                '(' => end_score = end_score * 5 + 1,
                '[' => end_score = end_score * 5 + 2,
                '{' => end_score = end_score * 5 + 3,
                '<' => end_score = end_score * 5 + 4,
                _ => unreachable!("nice input guy"),
            };
        }
        completion_scores.push(end_score);
    }
    println!("score: {}", score);
    completion_scores.sort();
    println!("mid completion score: {}", completion_scores[completion_scores.len()/2]);
}
