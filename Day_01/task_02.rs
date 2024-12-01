use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

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
    let mut count_of_appearances = 0;

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

    list1.sort();
    list2.sort();

   // Create a HashMap to count occurrences in list2
   let mut count_map: HashMap<i32, usize> = HashMap::new();
   for &num in &list2 {
       *count_map.entry(num).or_insert(0) += 1;
   }

   // Iterate over list1 and print the count of each number in list2
   for &num in &list1 {
       let count = count_map.get(&num).unwrap_or(&0);
       count_of_appearances += num * (*count as i32);
   }

   println!("The similarity score is {}", count_of_appearances);

   Ok(())
}