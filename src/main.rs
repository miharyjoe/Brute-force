use reqwest::StatusCode;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
extern crate num_cpus;

async fn check_path(base_url: String, word_list: Arc<Mutex<Vec<String>>>) {
    let client = reqwest::Client::new();

    loop {
        let word = {
            let mut list = word_list.lock().unwrap();
            if list.is_empty() {
                break;
            }
            list.pop().unwrap()
        };

        let url = format!("{}/{}", base_url, word);
        let response = client.get(&url).send().await;

        match response {
            Ok(res) => match res.status() {
                StatusCode::OK | StatusCode::INTERNAL_SERVER_ERROR => {
                    println!("Found: {}", url)
                }
                _ => {}
            },
            Err(_) => {}
        }
    }
}

fn main() -> Result<(), io::Error> {
    let base_url = "http://127.0.0.1:5000"; // Replace with your target URL
    let word_list_file = "dir_list.txt"; // Replace with the path to your wordlist file

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
        let base_url_clone = base_url.to_string();
        let handle = thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(check_path(base_url_clone, chunk));
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
