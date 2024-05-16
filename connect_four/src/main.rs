

use players::Players;
mod players;

const ROWS: usize = 6;
const COLUMNS: usize = 7;



type Board = [[char; COLUMNS]; ROWS];

fn main() {
    let mut board: Board = [[' '; COLUMNS]; ROWS];
    let mut current_player = Players::X; // Player X starts

    loop {
        print_board(&board);
        println!("Player {}, enter column (0-6): ", current_player);
        
        let mut col_str = String::new();
        std::io::stdin().read_line(&mut col_str).expect("Failed to read line");

        let col: usize = match col_str.trim().parse() {
            Ok(num) if num < COLUMNS => num,
            _ => continue, // Re-prompt on invalid input
        };

        if let Some(free_row) = find_open_slot_in_row(&board, col){
            board[free_row][col] = Into::<char>::into(current_player.clone())
        } else {
            println!("Column full, try a different one.");
        }

        current_player = if current_player == Players::X { Players::O } else { Players::X }; 

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


fn find_open_slot_in_row(board: &Board, col: usize) -> Option<usize> {
    for row in (0..6).rev() {
        if board[row][col] == ' ' {
            return Some(row);
        }
    }
    return None;
}
