// Demonstrating functions
// by implement algorithm to find the greatest common divisor(GCD)

// Utility function to determine the GCD
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        // Notice that we didn't add the RETURN keyword and didn't add the semicolon here
        // This is the RUST way of saying we want to return this expression
        gcd(b, a % b)
    } else {
        a
    }
}

// Driver function
fn main() {
    dbg!(gcd(143, 52));
} 
