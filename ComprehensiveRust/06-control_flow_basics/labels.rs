// Demonstrating labels

fn main() {
    let s = [[5, 6, 7], [8, 9 , 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 15;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            dbg!(elements_searched);
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);
}
