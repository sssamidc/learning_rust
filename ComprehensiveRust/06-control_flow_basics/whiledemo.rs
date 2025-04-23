// Demonstrating while loops
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
        dbg!(x);
    }
    dbg!(x);
}
