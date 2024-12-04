use std::env;
use std::fs::File;
use std::io::{self, BufRead};

const WORD: &str = "XMAS";
const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),   // Right
    (1, 0),   // Down
    (0, -1),  // Left
    (-1, 0),  // Up
    (1, 1),   // Down-Right
    (1, -1),  // Down-Left
    (-1, 1),  // Up-Right
    (-1, -1), // Up-Left
];

fn is_word_at(grid: &[Vec<char>], word: &str, row: usize, col: usize, dir: (isize, isize)) -> bool {
    let word_len = word.len();
    let (dr, dc) = dir;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for i in 0..word_len {
        let r = row as isize + i as isize * dr;
        let c = col as isize + i as isize * dc;

        if r < 0 || r >= rows || c < 0 || c >= cols || grid[r as usize][c as usize] != word.chars().nth(i).unwrap() {
            return false;
        }
    }
    true
}

fn find_word(grid: &[Vec<char>], word: &str) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &dir in &DIRECTIONS {
                if is_word_at(grid, word, row, col, dir) {
                    positions.push((row, col));
                }
            }
        }
    }
    positions
}

/// Prints the help message with usage instructions.
fn print_help() {
    println!("Usage: task_01 -f <file_path>");
    println!("-f <file_path> : Specify the path to the dataset file.");
    println!("-h, --help     : Print this help message.");
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut file_path = String::new();

    // Check for help flag
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_help();
        return Ok(());
    }

    // Check for file path flag and get the file path
    if let Some(index) = args.iter().position(|x| x == "-f") {
        if index + 1 < args.len() {
            file_path = args[index + 1].clone();
        } else {
            eprintln!("Error: No file path provided after -f parameter");
            std::process::exit(1);
        }
    } else {
        eprintln!("Error: -f parameter not found");
        print_help();
        std::process::exit(1);
    }

    // Open the file
    let input = File::open(&file_path)?;
    let buffered = io::BufReader::new(input);

    // Initialize a list of lists to store the dataset
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Read the file line by line
    for line in buffered.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        grid.push(chars);
    }

    // Find the word "XMAS" in the grid
    let positions = find_word(&grid, WORD);

    // Print the positions where the word was found
    for (row, col) in &positions {
        println!("Found '{}' at ({}, {})", WORD, row, col);
    }

    println!("count of words: {}", positions.len());
    Ok(())
}