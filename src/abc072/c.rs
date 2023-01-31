use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n:usize,
        a:[usize;n]
    }

    let mut count_hash = HashMap::<usize, usize>::new();
    for x in a {
        if x != 0 {
            let counter = count_hash.entry(x - 1).or_insert(0);
            *counter += 1;
        }
        let counter = count_hash.entry(x).or_insert(0);
        *counter += 1;

        if x != 100_000 {
            let counter = count_hash.entry(x + 1).or_insert(0);
            *counter += 1;
        }
    }

    let ans = count_hash.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("{}", ans.1);
}
