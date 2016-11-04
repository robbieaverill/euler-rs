// The sum of the squares of the first ten natural numbers is,
//     1(2) + 2(2) + ... + 10(2) = 385
//
// The square of the sum of the first ten natural numbers is,
//     (1 + 2 + ... + 10)2 = 552 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main () {
    let limit: i32 = 100;
    let mut sum_squares: i32 = 0;
    let mut sum: i32 = 0;

    for i in 1..limit + 1 {
        sum += i;
        sum_squares += i.pow(2);
    }

    let difference: i32 = sum.pow(2) - sum_squares;

    println!("Sum of first {} natural numbers' squares is: {}", limit, sum_squares);
    println!("Sum of all natural numbers, then squared is: {}", sum.pow(2));
    println!("Difference: {}", difference)
}
