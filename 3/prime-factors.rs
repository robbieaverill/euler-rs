// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143?

/**
 * Checks if `number` is a prime number
 */
fn is_prime_number(number: i64) -> bool {
    if number <= 1 {
        return false;
    } else if number <= 3 {
        return true;
    } else if number % 2 == 0 || number % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (number + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true
}

/**
 * Checks whether `factor` is a factor of `factor_of`
 */
fn is_factor_of(factor: i64, factor_of: i64) -> bool {
    factor_of % factor == 0
}

/**
 * Returns the highest prime factor for `number`
 */
fn highest_prime_factor(number: i64) -> i64 {
    for i in (2..number).rev() {
        if is_factor_of(i, number) && is_prime_number(i) {
            return i;
        }
    }
    return 2;
}

/**
 * Gets the highest prime factor for a given number.
 *
 * NOTE: This was more of an exercise than anything. It should not be considered as efficient.
 */
fn main () {
    // println!("1: {:?}", is_prime_number(1));
    // println!("2: {:?}", is_prime_number(2));
    // println!("3: {:?}", is_prime_number(3));
    // println!("4: {:?}", is_prime_number(4));
    // println!("5: {:?}", is_prime_number(5));
    // println!("6: {:?}", is_prime_number(6));
    // println!("7: {:?}", is_prime_number(7))
    let number: i64 = 600851475143;
    println!("Input: {}", number);

    let highest_prime_factor = highest_prime_factor(number);
    println!("Highest prime of {} is {}", number, highest_prime_factor)
}
