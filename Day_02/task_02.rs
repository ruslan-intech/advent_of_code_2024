use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn is_monotonic<T: PartialOrd>(lst: &[T]) -> bool {
    if lst.len() <= 1 {
        return true;
    }

    let mut is_increasing = true;
    let mut is_decreasing = true;

    for i in 0..lst.len() - 1 {
        if let Some(ordering) = lst[i].partial_cmp(&lst[i + 1]) {
            if ordering != std::cmp::Ordering::Less {
                is_increasing = false;
            }
            if ordering != std::cmp::Ordering::Greater {
                is_decreasing = false;
            }
        } else {
            return false; // Non-comparable elements
        }
    }

    is_increasing || is_decreasing
}

fn is_safe(row: &[i32]) -> bool {
    is_monotonic(row) && row.windows(2).all(|w| (w[0] - w[1]).abs() < 4 && (w[0] - w[1]) != 0)
}

fn can_be_safe_by_removing_one(row: &[i32]) -> bool {
    for i in 0..row.len() {
        let mut new_row = row.to_vec();
        new_row.remove(i);
        if is_safe(&new_row) {
            return true;
        }
    }
    false
}

/// Prints the help message with usage instructions.
fn print_help() {
    println!("Usage: task_02 -f <file_path>");
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
    let mut dataset: Vec<Vec<i32>> = Vec::new();

    // Read the file line by line
    for line in buffered.lines() {
        let line = line?;
        let columns: Vec<i32> = line.split_whitespace()
                                    .filter_map(|s| s.parse::<i32>().ok())
                                    .collect();
        dataset.push(columns);
    }

    let mut safe_count: i32 = 0;

    // Check if each row is safe or can be made safe by removing one level
    for row in dataset.iter() {
        let is_safe_as_is = is_safe(&row);
        let can_be_safe = is_safe_as_is || can_be_safe_by_removing_one(&row);

        if can_be_safe {
            safe_count += 1;
        }

        println!("Row: {:?} is safe: {}", row, can_be_safe);
    }

    println!("Sum of safe counts: {}", safe_count);

    Ok(())
}