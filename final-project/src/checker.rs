use std::time::{Instant, SystemTime, Duration};
use std::sync::mpsc;
use std::thread;
use ureq;
use crate::models::WebsiteStatus;

pub fn check_websites(urls: Vec<String>, timeout: Duration, num_threads: usize) -> Vec<WebsiteStatus> {
    let (sender, receiver) = mpsc::channel();
    let chunk_size = (urls.len() + num_threads - 1) / num_threads;

    let mut handles = vec![];
    for chunk in urls.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let sender = sender.clone();
        let timeout = timeout.clone();

        let handle = thread::spawn(move || {
            for url in chunk {
                let start = Instant::now();
                let timestamp = SystemTime::now();

                let response = ureq::get(&url).timeout(timeout).call();
                let status = match response {
                    Ok(resp) => Ok(resp.status()),
                    Err(err) => Err(err.to_string()),
                };
                let response_time = start.elapsed();

                sender.send(WebsiteStatus {
                    url,
                    status,
                    response_time,
                    timestamp,
                }).unwrap();
            }
        });

        handles.push(handle);
    }

    drop(sender);

    let mut results = vec![];
    for result in receiver {
        results.push(result);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    results
}
