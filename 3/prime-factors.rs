// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143?

fn get_primes(boundary: i32) -> Vec<i32> {
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

// fn get_prime_factors() {
//
// }

fn main () {
    let primes = get_primes(15);
    println!("Prime numbers below 15: {:?}", primes)
}
