//is this even memoization? i don't think so
//it is working!

use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("{}", memoized_factorial(5));
    println!("{}", now.elapsed().as_micros());
    let now2 = Instant::now();
    println!("{}", memoized_factorial(5));
    println!("{}", now2.elapsed().as_micros());
}

fn memoized_factorial(n: i32) -> i32 {
    let mut pre_computed_fact = (0, 1);
    let mut x = 1;
    if (pre_computed_fact.0 != 0 && pre_computed_fact.0 == n) {
        pre_computed_fact.1
    } else {
        for i in 1..n + 1 {
            x = x * i;
        }
        pre_computed_fact.0 = n;
        pre_computed_fact.1 = x;
        x
    }
}
