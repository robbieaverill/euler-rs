// If we list all the natura numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9.
// The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000.

fn calculate(boundary: i32) {
    // Initialize a result vector containing multiples of 3 or 5
    let mut results = Vec::new();
    let mut sum: i32 = 0;

    for i in 1..boundary {
        if i % 3 == 0 || i % 5 == 0 {
            results.push(i);
            sum += i;
        }
    }

    println!("{}", sum)
}

fn main () {
    println!("Sum of multiples of 3 or 5 below 10:");
    calculate(10);

    println!("Sum of multiples of 3 or 5 below 1000:");
    calculate(1000)
}
