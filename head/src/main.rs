use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let number_of_lines = get_number_of_lines(&args);
    let files: Vec<&String> = get_files(&args);

    for (_, file) in files.iter().enumerate() {
        println!("==> {} <==", file);

        if let Err(e) = read_file_line_by_line(file, number_of_lines) {
            eprintln!("Error reading file {}: {}", file, e);
        }
    }
}

// Extracts the number of lines to read from the command-line arguments.
// If the "-n" option is not provided, defaults to 10.
fn get_number_of_lines(args: &Vec<String>) -> usize {
    let nOption = args.iter().find(|i| i.contains("-n"));

    match nOption {
        Some(v) => v.split_once("-n").unwrap().1.parse().unwrap_or(10),
        None => 10,
    }
}

// Extracts the list of file names from the command-line arguments,
// excluding any arguments that contain the "-n" option.
fn get_files(args: &Vec<String>) -> Vec<&String> {
    args.iter().filter(|arg| !arg.contains("-n")).collect()
}

// Reads the specified number of lines from the given file and prints them to the console.
fn read_file_line_by_line(filename: &str, n: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines().take(n) {
        println!("{}", line?);
    }

    println!("");

    Ok(())
}
