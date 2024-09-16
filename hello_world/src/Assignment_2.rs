fn main() {
    // Define an array of 10 integers
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Function to check if a number is even
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // For loop to analyze each number in the array
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // While loop to calculate the sum of all numbers
    let mut sum = 0;
    let mut index = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of all numbers: {}", sum);

    // Loop to find the largest number
    let mut largest = numbers[0];

    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }

    println!("Largest number: {}", largest);
}
