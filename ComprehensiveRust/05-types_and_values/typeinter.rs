// Rust will look at how the variable is used
// to determine its type

// Function expects a u32 type variable as input param
fn takes_u32(x: u32) {
    println!("u32: {x}");
}


// Function expects a i8 type variable as input param
fn takes_i8(x: i8) {
    println!("i8: {x}");
}


fn main() {
    let x = 10;
    let y = 20;
    
    // Some function calls
    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}