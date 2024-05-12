use std::io::stdin;

const ROWS: usize = 6;
const COLUMNS: usize = 7;

#[derive(Debug, Default)]
enum Players {
    #[default]
    X,
    O
}

type Board = [[char; COLUMNS]; ROWS];

fn main() {
    let mut board: Board = [[' '; COLUMNS]; ROWS];
    let mut current_player = 'X'; // Player X starts

    loop {
        print_board(&board);
        println!("Player {}, enter column (0-6): ", current_player);
        
        let mut col_str = String::new();
        std::io::stdin().read_line(&mut col_str).expect("Failed to read line");

        let col: usize = match col_str.trim().parse() {
            Ok(num) if num < COLUMNS => num,
            _ => continue, // Re-prompt on invalid input
        };

    }
}

fn print_board(board: &Board) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("[{}]", cell);
        }
        println!();
    }
    println!();
}
