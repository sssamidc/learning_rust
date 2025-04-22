// Demonstrating match

fn main() {
    let val = 123;
    match val {
        1 => println!("one"),
        10 => println!("two"),
        100 => println!("one hundred"),
        _ => {
            println!("something else")
        }
    }
}
