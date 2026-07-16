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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <filename> --top[OPTIONAL] <num>");
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

    let mut top: i32 = 0;

    if args.len() > 2 {
        if args.len() != 4 {
            eprintln!("Usage: <filename> --top[OPTIONAL] <num>");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )));
        }
        if args[2] != "--top" {
            eprintln!("Invalid input: {}", args[2]);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid input",
            )));
        }
        top = match args[3].trim().parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Error: Invalid Input, expected a number after `--top`",
                )));
            }
        }
    }

    let file_data_vec: Vec<String> = read_lines(&args[1]);
    let mut file_data: HashMap<String, i32> = HashMap::new();

    for line in &file_data_vec {
        for word in line.split_whitespace() {
            *file_data.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut sorted_file_data: Vec<(&String, &i32)> = file_data.iter().collect();
    sorted_file_data.sort_by(|a, b| b.1.cmp(&a.1));

    println!("---- WORDCOUNT -----");
    println!("Word   Occurence Count");
    let mut counter = 0;
    for (word, occurence) in sorted_file_data.iter() {
        println!("{word}:    {occurence}");
        counter += 1;
        if counter == top && top != 0 {
            break;
        }
    }
    println!("Total Number of words: {}", sorted_file_data.len());
    Ok(())
}
