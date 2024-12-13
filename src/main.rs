mod worker;
mod types;

use std::env;
use serde_json::to_string_pretty;
use worker::check_website;

fn main() {
    // Collect URLs from command-line arguments
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: monitor <url1> <url2> ...");
        std::process::exit(1);
    }

    let mut results = Vec::new();

    for url in args {
        let result = check_website(&url, 5); // 5 seconds timeout
        results.push(result);
    }

    // Serialize results to JSON and print to stdout
    match to_string_pretty(&results) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Failed to serialize results: {}", e),
    }
}
