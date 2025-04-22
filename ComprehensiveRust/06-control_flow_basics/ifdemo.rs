// Demonstrating basic if-else if-else blocks

fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge!");
    }

    // Variable assignment with if statements
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("Size: {}", size);
}
