
// #1 
// fn main() {
//     let operation = |a: i32, b: i32| -> i32 {
//         // Your implementation here
//         a * b
//     };
//     println!("Result: {:?}", operation(10, 5));
// }

// #2
// fn track_changes() {
//     let mut tracker = 0;
//     let mut update = || {
//         // Your implementation here
//         tracker += 1;
//         println!("Updated: {} times", tracker);
//     };
//     update();
//     update();
// }

// fn main() {
//     track_changes();
// }

// #3
// fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     vec.into_iter().map(f).collect()
// }

// fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x));
//     }
//     result
// }

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
    
//     // process_vector_with_map(vec, f)  // Use either implementation:
//     process_vector_with_for_loop(vec, f)
// }

// fn main() {
//     let numbers = vec![1, 2, 3];

//     let doubled = process_vector(numbers.clone(), |x| x * 2); // Multiply each number by 2

//     let replaced = process_vector(numbers, |x| {
//         if x > 2 {
//             0 // Replace with 0 if greater than 2
//         } else {
//             x // Keep the number otherwise
//         }
//     });

//     println!("Doubled: {:?}", doubled);
//     println!("Replaced: {:?}", replaced);
// }

// #5
use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    result: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    // Constructor to initialize the cache with a computation closure
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            result: None,
        }
    }

    // Method to get the result, either computed or cached
    fn get_result(&mut self) -> String {
        match &self.result {
            Some(value) => {
                println!("Retrieved from cache instantly!");
                value.clone()
            }
            None => {
                println!("Computing (this will take 2 seconds)...");
                thread::sleep(Duration::from_secs(2));
                let value = (self.computation)();
                self.result = Some(value.clone());
                value
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Performing expensive computation...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());
    
    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

