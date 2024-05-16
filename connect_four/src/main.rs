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
        std::io::stdin()
            .read_line(&mut col_str)
            .expect("Failed to read line");

        let col: usize = match col_str.trim().parse() {
            Ok(num) if num < COLUMNS => num,
            _ => continue, // Re-prompt on invalid input
        };

        let free_row = match find_open_slot_in_row(&board, col) {
            Some(row) => row,
            None => {
                println!("Column full, try a different one.");
                continue;
            }
        };
        board[free_row][col] = Into::<char>::into(current_player.clone());

        match check_for_win(&board) {
            Some(winner) => {
                print_board(&board);
                println!("We have a winner: {}", winner);
                break;
            }
            None => {
                if check_for_draw(&board) {
                    println!("We have a draw!");
                    break;
                }
            }
        }

        current_player = if current_player == Players::X {
            Players::O
        } else {
            Players::X
        };
    }
}

fn check_for_win(board: &Board) -> Option<Players> {
    let mut r: String = " ".to_string();
    // Horizontal check
    for row in 0..ROWS {
        for col in 0..COLUMNS {
            r = r + &board[row][col].to_string()
        }

        if r.contains("XXXX") {
            return Some(Players::X);
        } else if r.contains("OOOO") {
            return Some(Players::O);
        }

        r = " ".to_string();
    }

    // Vertical check
    for col in 0..COLUMNS {
        for row in 0..ROWS {
            r = r + &board[row][col].to_string()
        }

        if r.contains("XXXX") {
            return Some(Players::X);
        } else if r.contains("OOOO") {
            return Some(Players::O);
        }

        r = " ".to_string();
    }

    // Diagonical check top-left to bottom-right
    for row in 0..ROWS - 3 {
        for col in 0..COLUMNS - 3 {
            if board[row][col] == board[row + 1][col + 1]
                && board[row][col] == board[row + 2][col + 2]
                && board[row][col] == board[row + 3][col + 3]
                && (board[row][col] == Players::X.into() || board[row][col] == Players::O.into())
            {
                return if board[row][col] == Players::X.into() {
                    Some(Players::X)
                } else {
                    Some(Players::O)
                };
            }
        }
    }

    // top-right to bottom-left
    for row in 0..ROWS - 3 {
        for col in 3..COLUMNS {
            if board[row][col] == board[row + 1][col - 1]
                && board[row][col] == board[row + 2][col - 2]
                && board[row][col] == board[row + 3][col - 3]
                && (board[row][col] == Players::X.into() || board[row][col] == Players::O.into())
            {
                return if board[row][col] == Players::X.into() {
                    Some(Players::X)
                } else {
                    Some(Players::O)
                };
            }
        }
    }
    None
}

fn check_for_draw(board: &Board) -> bool {
    board.iter().all(|row| row.iter().all(|cell| *cell != ' '))
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
