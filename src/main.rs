mod worker; // Imports worker module with logic, "check_website" to check websites
mod types;  // Imports types to define data structures in the application

use std::env; 
use serde_json::to_string_pretty; // Formats the data as pretty printed Json
use worker::check_website; // Imports check_website from worker

fn main() {
    // Collect URLs from command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {   // Check if no URLs are given
        eprintln!("Usage: monitor <url1> <url2> ..."); // Prints out error message
        std::process::exit(1); // Exit the program with 1 status, or non-zero, to indicate error
    }

    let mut results = Vec::new(); // Initialize an empty vector to store the resutls of the website checks

    for url in args { // Iterate through each URL given
        let result = check_website(&url, 5); // 5 seconds timeout
        results.push(result); // Push result to the results vector
    }

    match to_string_pretty(&results) { // Serialize results to JSON and print to stdout
        Ok(json) => println!("{}", json), // If successful, print the JSON
        Err(e) => eprintln!("Failed to serialize results: {}", e), // Otherwise, print error to standard error
    }
}
