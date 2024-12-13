use crate::config::Config;
use crate::types::WebsiteStatus;
use crate::worker::check_website;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

pub fn start_monitoring(config: Config) -> Result<(), String> {
    let (tx, rx): (Sender<WebsiteStatus>, Receiver<WebsiteStatus>) = mpsc::channel();
    let mut handles = Vec::new();

    for i in 0..config.num_threads {
        let tx_clone = tx.clone();
        let urls = config.urls.clone();
        let timeout = config.timeout;

        let handle = thread::spawn(move || {
            for url in urls.iter().skip(i).step_by(config.num_threads) {
                let result = check_website(url, timeout);
                if let Err(_) = tx_clone.send(result) {
                    eprintln!("Failed to send result for URL: {}", url);
                }
            }
        });

        handles.push(handle);
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
