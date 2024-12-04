use std::io;
use std::env;
use std::fs::File;
use std::fs::read_to_string;

fn find_xmas(grid: &Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for y in 1..height-1 {
        for x in 1..width-1 {
            if grid[y][x] != 'A' { continue; }
            if (check_diagonal(&grid, x, y, -1, -1, 'M') &&
                check_diagonal(&grid, x, y, 1, 1, 'S') ||
                check_diagonal(&grid, x, y, -1, -1, 'S') &&
                check_diagonal(&grid, x, y, 1, 1, 'M')) &&
               (check_diagonal(&grid, x, y, -1, 1, 'M') &&
                check_diagonal(&grid, x, y, 1, -1, 'S') ||
                check_diagonal(&grid, x, y, -1, 1, 'S') &&
                check_diagonal(&grid, x, y, 1, -1, 'M'))
            {
                count += 1;
            }
        }
    }
    count
}

fn check_diagonal(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    dy: i32,
    dx: i32,
    expected: char
) -> bool {
    let new_y = y as i32 + dy;
    let new_x = x as i32 + dx;
    
    new_y >= 0 && new_y < grid.len() as i32 &&
    new_x >= 0 && new_x < grid[0].len() as i32 &&
    grid[new_y as usize][new_x as usize] == expected
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let mut file_path = String::new();

    // Check for help flag
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        println!("Usage: task_02 -f <file_path>");
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
        println!("Usage: task_02 -f <file_path>");
        std::process::exit(1);
    }

    // Open the file
    let text = read_to_string(&file_path)?;

    let grid: Vec<Vec<char>> = text.trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    
    let result = find_xmas(&grid);
    println!("XMAS appears {} times", result);

    Ok(())
}