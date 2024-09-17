// Function to check if the guess is correct, too high, or too low
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0  // Correct guess
    } else if guess > secret {
        1  // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Set the secret number (hardcoded)
    let secret_number: i32 = 42;
    
    // Initialize variables for the guessing loop
    let mut guess_count = 0;
    let mut guess: i32;
    
    loop {
        // Increment the guess count
        guess_count += 1;
        
        // Simulate a user guess (replace with user input in a real game)
        guess = if guess_count == 1 { 30 } else if guess_count == 2 { 50 } else { 42 };

        // Check the guess using the check_guess function
        let result = check_guess(guess, secret_number);

        // Print result based on the return value of check_guess
        if result == 0 {
            println!("Correct! The secret number is {}.", secret_number);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }
    }
    
    // Print how many guesses it took
    println!("You guessed the secret number in {} attempts.", guess_count);
}
