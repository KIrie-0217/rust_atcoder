use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        _n:usize,
        a:usize,b:usize,
        k:usize,
        p:[usize;k]
    }

    let mut map_hash: HashMap<usize, bool> = HashMap::<usize, bool>::new();
    map_hash.insert(a, true);
    map_hash.insert(b, true);

    let mut ans: bool = true;
    for point in p {
        if map_hash.contains_key(&point) {
            ans = false;
            break;
        }
        map_hash.insert(point.clone(), true);
    }

    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
