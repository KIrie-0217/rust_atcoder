use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        _n:usize,
        s:Chars
    }

    let mut check: HashSet<Vec<i128>> = HashSet::<Vec<i128>>::new();
    let mut x: i128 = 0;
    let mut y: i128 = 0;

    let mut is_duplicate: bool = false;
    check.insert(vec![x, y]);
    for c in s {
        match &c {
            'R' => x += 1,
            'L' => x -= 1,
            'U' => y += 1,
            'D' => y -= 1,
            _ => unreachable!(),
        }
        if check.contains(&vec![x, y]) {
            is_duplicate = true;
            break;
        }

        check.insert(vec![x, y]);
    }

    if is_duplicate {
        println!("Yes");
    } else {
        println!("No");
    }
}
