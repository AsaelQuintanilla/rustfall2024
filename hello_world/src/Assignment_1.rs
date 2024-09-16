// Create a Rust program that converts temperatures between Fahrenheit and Celsius. The program should:

// Declare a constant for the freezing point of water in Fahrenheit (32°F).
// Implement two functions:
// fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
// celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
// In the main function:
// Declare a mutable variable with a temperature in Fahrenheit
// Convert it to Celsius using your function and print the result
// Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    // Declare a mutable variable for the starting Fahrenheit temperature
    let mut temp_f: f64 = 32.0;

    // Convert the starting temperature to Celsius and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);

    // Loop to convert and print the next 5 Fahrenheit temperatures
    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, temp_c);
    }
}