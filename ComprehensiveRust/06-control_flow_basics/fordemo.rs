// Demonstrating for loops

fn main() {
    // notice here that we're printing the variables till 4 and NOT 5
    // Begin: inclusive, End: exclusive
    for x in 1..5 {
        dbg!(x);
    }

    for elem in [2, 4, 8, 16, 32] {
        dbg!(elem);
    }
}
