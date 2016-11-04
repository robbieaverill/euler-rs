// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

/**
 * Finds the largest palindromic number where the factors are both two digits
 */
fn largest_palindrome(digits: i32) -> i32 {
    // Get the highest number to start testing from
    let start = (10 as f32).powi(digits) as i32;
    let mut palindromes = Vec::new();

    // Find all the palinromes
    for i in (1..start).rev() {
        for x in (1..start).rev() {
            let current_number = i * x;
            if is_palindrome(current_number) {
                // println!("Factors: {}, {} and result {}", i, x, current_number);
                palindromes.push(current_number);
            }
        }
    }

    // Sort them
    palindromes.sort();

    // Return the highest one
    match palindromes.pop() {
        Some(val) => val as i32,
        None => 0
    }
}

/**
 * Decides if a number is palindromic by comparing it forwards and backwards as a string
 */
fn is_palindrome(number: i32) -> bool {
    let forward = number.to_string();
    let backward = forward.chars().rev().collect::<String>();
    return forward == backward;
}

fn main () {
    println!("Largest palindrome number from the product of two 2-digit numbers is: {}", largest_palindrome(2));
    println!("Largest palindrome number from the product of two 3-digit numbers is: {}", largest_palindrome(3))
}
