use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::read_to_string;

// read each line from the file and return a vector
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}

// program entry point
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: <filename> --top <num>");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid input",
        )));
    }

    if !fs::exists(&args[1])? {
        eprintln!("File not found: {}", args[1]);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found",
        )));
    }

    if args.len() > 2 {
        if args[2] != "--top" {
            eprintln!("Invalid input: {}", args[2]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )));
        }
    }

    let file_data_vec: Vec<String> = read_lines(&args[1]);
    let mut file_data = HashMap::new();

    for data in &file_data_vec {
        file_data.insert(data, data);
    }
    Ok(())
}
