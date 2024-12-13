use crate::config::Config; // Import Config struct from config moduel
use crate::types::WebsiteStatus; // Import WebsiteStatus type from types module
use crate::worker::check_website; // Import check_website function from worker module
use std::sync::mpsc::{self, Sender, Receiver}; 
use std::thread; // Imports ability for threading

pub fn start_monitoring(config: Config) -> Result<(), String> {
    // Creates channel for sending/receiving website status updates
    let (tx, rx): (Sender<WebsiteStatus>, Receiver<WebsiteStatus>) = mpsc::channel();
    let mut handles = Vec::new(); // Vector for storing thread handles

    for i in 0..config.num_threads { // Spawn threads based on number of threads in config
        let tx_clone = tx.clone(); // Clone the sender for each thread
        let urls = config.urls.clone(); // Clone the lsit of URLs to share between threads
        let timeout = config.timeout; // Get the timeout value from config

        let handle = thread::spawn(move || { // Spawn new thread to check a subset of the URLs
            for url in urls.iter().skip(i).step_by(config.num_threads) {
                let result = check_website(url, timeout); // Checls the website with the timeout given
                if let Err(_) = tx_clone.send(result) { // Sends the result to the main thread. If fails, it logs an error
                    eprintln!("Failed to send result for URL: {}", url);
                }
            }
        });

        handles.push(handle); // Store the thread handle for later usage
    }

    // Collect results
    drop(tx); // Close the sender so the receiver knows when to stop
    for received in rx {
        println!("{:?}", received); // Print results to the console
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().map_err(|_| "Thread panicked")?;
    }

    Ok(())
}
