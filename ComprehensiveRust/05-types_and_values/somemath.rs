// Demonstrating basic math operation

// Utility function to calculate the cross product
fn crossproduct(a: i32, b: i32, c: i32) -> i32 {
    return (a * b) + (b * c) + (c * a);
}

// Driver function
fn main() {
    println!("result: {}", crossproduct(123, 456, 789));
}