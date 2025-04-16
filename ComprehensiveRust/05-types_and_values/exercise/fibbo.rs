// Solution to programming challenge 5.1

fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return 1;
    } else {
        // The recursive case.
        return fib(n - 1) + fib(n - 2);
    }
}

// Driver function
fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
