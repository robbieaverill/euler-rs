// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143?

/**
 * Returns the prime numbers lower than the given boundary
 */
fn get_primes(boundary: i64) -> Vec<i64> {
    let mut result = Vec::new();

    for i in 2..boundary {
        // Innocent until proven guilty!
        let mut is_prime = true;

        if i % 2 == 0 {
            // Skip even numbers.
            continue;
        }

        for n in 2..boundary {
            if i % n == 0 && i != n {
                // It's not a prime - i can be divided evenly into n parts (or it's just even)
                is_prime = false;
            }
        }

        // It's a prime number. Has no common divisors other than 1 and itself
        if is_prime {
            result.push(i);
        }
    }

    return result;
}

/**
 * Returns the prime factors for a given number as a vector
 */
fn get_prime_factors(number: i64) -> Vec<i64> {
    let prime_numbers = get_primes(number);
    let mut result = Vec::new();

    for i in 0..prime_numbers.len() {
        if number % prime_numbers[i] == 0 {
            // Found a prime factor
            result.push(prime_numbers[i]);
        }
    }

    return result
}

/**
 * Gets the highest prime factor for a given number.
 *
 * NOTE: This was more of an exercise than anything. It has INCREDIBLY poor performance, and is
 * processing far more numbers than it needs to. For example - you could easily square root the
 * number to get a starting point for the highest prime factor. Nothing higher than the sqrt would
 * be a prime factor. Thoughts for next time...
 */
fn main () {
    let number: i64 = 600851475143;
    let factors = get_prime_factors(number);
    println!("Input: {}", number);
    println!("Result: {:?}", factors);

    println!("Largest prime factor is {:?}", factors[factors.len() - 1])
}
