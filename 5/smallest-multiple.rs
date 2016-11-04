// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

/**
 * Given a number and a limit, check whether `number` is evenly divisible into each number between 1 and `into`
 */
fn is_evenly_divisible_into(number: i32, into: i32) -> bool {
    for i in 1..into {
        if number % i != 0 {
            return false;
        }
    }
    return true;
}

fn main () {
    let mut i = 1;
    loop {
        if is_evenly_divisible_into(i, 20) {
            println!("Found answer: {}", i);
            break;
        }
        i += 1;
    }
    println!("Done...")
}
