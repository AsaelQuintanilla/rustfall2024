pub struct Config {
    pub num_threads: usize,
    pub timeout: u64,
    pub max_retries: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            num_threads: 4,
            timeout: 5,
            max_retries: 3,
        }
    }
}
