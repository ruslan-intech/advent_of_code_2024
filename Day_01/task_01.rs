use std::env;
use std::fs::File;
use std::io::{self, BufRead};

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

    // Initialize lists to store the columns
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    // Read the file line by line
    for line in buffered.lines() {
        let line = line?;
        let mut columns = line.split_whitespace();
        if let (Some(col1), Some(col2)) = (columns.next(), columns.next()) {
            if let (Ok(num1), Ok(num2)) = (col1.parse::<i32>(), col2.parse::<i32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    // Sort both lists
    list1.sort();
    list2.sort();

    // Calculate absolute differences between corresponding elements
    let differences: Vec<i32> = list1.iter().zip(list2.iter())
                                     .map(|(a, b)| (a - b).abs())
                                     .collect();

    // Calculate the sum of the differences
    let sum: i32 = differences.iter().sum();
    println!("Sum of differences: {}", sum);

    Ok(())
}