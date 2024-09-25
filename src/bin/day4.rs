const INPUT: &'static str = include_str!("../data/input_4.txt");

fn main () {

    let mut input_string = INPUT.split("\n\n");

    let numbers: Vec<u32> = input_string
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    println!("{:?}", &numbers);
 

    // construct boards
    let mut boards: Vec<Board> = vec![];
    for item in &mut input_string {
        let mut board: Vec<u32> = vec![];

        // long str
        let current_table = item
            .split('\n')
            .collect::<Vec<&str>>();

        // long str to vec
        for row in current_table {
            board.extend(row.split_whitespace()
                .map(|n| n.parse::<u32>()
                .unwrap())
                .collect::<Vec<u32>>()
                .iter()
            );   
        }

        boards.push(Board {
            board: board,
            marked: vec![false; 25]
        });
    }

    // mark 
    'outer: for num in &numbers { 
        'inner: for board in &mut boards {
            board.mark(*num);
            if board.is_win() {
                println!("{:?} WINS", board);
                println!("Part 1: {:?}", board.score() * num);
                break 'outer;
            }
            
        }
    }

    for n in &numbers {
        boards.iter_mut().for_each(|b| b.mark(*n));

        if boards.len() == 1 && boards[0].is_win() {
            println!("Part 2: {}", boards[0].score() * n);
        }

        boards.retain(|b| !b.is_win());
    }
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<u32>,
    marked: Vec<bool>
}

impl Board {
    fn mark(&mut self, marked: u32) {
        if let Some(pos) = self.board.iter().position(|n| *n == marked) {
            self.marked[pos] = true;
        }
    }

    fn is_win(&self) -> bool {
        for row in 0..5 {
            let start = row * 5;
            if (start..start + 5).all(|i| self.marked[i]) {
                return true;
            }
        }
        for col in 0..5 {
            if (col..5 * 5)
                .step_by(5)
                .all(|i| self.marked[i])
            {
                return true;
            }
        }
        false
    }



    fn score(&self) -> u32 {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, n)| (!self.marked[i]).then(|| n))
            .sum()
    }
}
