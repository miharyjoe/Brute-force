// Import necessary crates and modules
use reqwest::StatusCode;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
extern crate num_cpus; // Import num_cpus crate for CPU core detection
use clap::{App, Arg}; // Import clap crate for command-line argument parsing

// Asynchronous function to check paths
async fn check_path(base_url: String, word_list: Arc<Mutex<Vec<String>>>) {
    let client = reqwest::Client::new(); // Create a reqwest client

    // Loop to continuously check paths
    loop {
        let word = {
            let mut list = word_list.lock().unwrap(); // Lock the word list for thread safety
            if list.is_empty() {
                break; // Break the loop if word list is empty
            }
            list.pop().unwrap() // Pop a word from the list
        };

        let url = format!("{}/{}", base_url, word); // Construct the URL to check
        let response = client.get(&url).send().await; // Send a GET request to the URL

        // Match on the response status
        match response {
            Ok(res) => match res.status() {
                StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR => {
                    println!("Found: {}", url) // Print URL if status is OK or Internal Server Error
                }
                _ => {} // Do nothing for other status codes
            },
            Err(_) => {} // Ignore errors
        }
    }
}

// Main function
fn main() -> Result<(), io::Error> {
    // Parse command-line arguments
    let matches = App::new("URL Checker")
        .version("1.0")
        .author("mihary joel")
        .about("Checks URLs for existence")
        .arg(
            Arg::with_name("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("Sets the base URL")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("wordlist")
                .short('w')
                .long("wordlist")
                .value_name("FILE")
                .help("Sets the path to the wordlist file")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let base_url = matches.value_of("url").unwrap(); // Get the base URL from command-line arguments
    let word_list_file = matches.value_of("wordlist").unwrap(); // Get the wordlist file path

    // Read wordlist from file
    let file = File::open(word_list_file)?;
    let reader = io::BufReader::new(file);
    let word_list: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Determine the number of threads based on CPU cores
    let num_threads = num_cpus::get();

    // Divide the wordlist into chunks for each thread
    let chunk_size = (word_list.len() + num_threads - 1) / num_threads;
    let word_list_chunks: Vec<_> = word_list
        .chunks(chunk_size)
        .map(|chunk| Arc::new(Mutex::new(chunk.to_vec())))
        .collect();

    // Spawn threads
    let mut handles = vec![];
    for chunk in word_list_chunks {
        let base_url_clone = base_url.to_string(); // Clone base URL for each thread
        let handle = thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(check_path(base_url_clone, chunk)); // Spawn a thread to check paths
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(()) // Return Ok if everything executed successfully
}
