// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?
//
// Accepts an argument for the number

use std::env;

/**
 * Checks if `number` is a prime number (borrowed from #3)
 */
pub fn is_prime_number(number: i64) -> bool {
    if number <= 1 {
        return false;
    } else if number <= 3 {
        return true;
    } else if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut i: i64 = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true
}

/**
 * Finds the nth prime number from 1 to n
 */
fn find_nth_prime(nth: i64) -> i64 {
    let mut prime_i = 0;
    for i in 1..100000000 { // let's give it SOME limit! no processor likes infinity
        if is_prime_number(i) {
            prime_i += 1;
        }

        if prime_i == nth {
            return i;
        }
    }
    return 0
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let nth: i64 = args[1].parse().expect("Must be an integer");
    let result = find_nth_prime(nth);

    println!("Nth prime ({}) is {}", nth, result);
    println!("Is prime: {:?}", is_prime_number(result))
}
