fn main() {
    // Define the secret number
    let secret_number: i32 = 7; // You can change this to any number you like

    // Variable to keep track of the number of guesses
    let mut guess_count = 0;

    // Loop to handle guessing
    loop {
        // Simulate a user input for the guess
        // In a real program, you would use something like std::io::stdin().read_line()
        // For simplicity, we're using hard-coded values here
        let mut guess: i32 = 5; // Start with an example guess (you can change this to test different values)

        // Update guess to simulate user input
        guess_count += 1;

        // Check the guess
        let result = check_guess(guess, secret_number);

        // Print feedback based on the result
        if result == 0 {
            println!("Congratulations! You've guessed the correct number.");
            break; // Exit the loop since the guess was correct
        } else if result == 1 {
            println!("Your guess is too high. Try again.");
        } else {
            println!("Your guess is too low. Try again.");
        }

        // Simulate a change in guess for demonstration (remove or replace with actual user input)
        guess = if guess < secret_number { guess + 1 } else { guess - 1 };
    }

    // Print the number of guesses taken
    println!("It took you {} guesses to find the correct number.", guess_count);
}
