use std::usize::MAX;

use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
        a:[usize;n]
    }
    let mut check_master: usize = 0;
    for i in 0..k {
        check_master += a[i];
    }

    let mut a_with_index: Vec<Vec<isize>> = vec![];
    for i in 0..n {
        a_with_index.push(vec![a[i] as isize, i as isize * (-1)]);
    }
    a_with_index.sort_by(|a, b| b.cmp(&a));

    let mut check: usize = 0;
    for i in 0..k {
        check += a_with_index[i][0] as usize;
    }
    if check == check_master {
        println!("-1");
        return;
    }

    let mut max_in_k: usize = MAX;
    let mut ans: usize = MAX;
    println!("{:?}", a_with_index);
    for a_it in a_with_index {
        if ((a_it[1] * -1) as usize >= k + 1) && (max_in_k != MAX) {
            ans = ans.min((a_it[1] * -1) as usize - max_in_k);
        }

        if (a_it[1] * -1) as usize <= k {
            max_in_k = max_in_k.max((a_it[1] * -1) as usize);
        }
    }

    println!("{}", ans);
}
