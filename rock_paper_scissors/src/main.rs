use std::env;

fn check_args(args: &Vec<String>) {
    let cli_args: Vec<String> = vec![
        String::from("controller"),
        String::from("player1"),
        String::from("player2"),
    ];

    if args.len() != 2 {
        panic!("Wrong number of arguments")
    }

    if !cli_args.contains(&args[1].to_string()) {
        panic!("Wrong choice")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_args(&args);
}
