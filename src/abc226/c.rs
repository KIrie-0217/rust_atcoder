use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n:usize
    }
    let mut t_vec: Vec<usize> = vec![];
    let mut k_vec: Vec<usize> = vec![];
    let mut a_vec: Vec<Vec<usize>> = vec![];
    for _ in 0..n {
        input! {
            t:usize,k:usize,
            a:[usize;k]
        }
        t_vec.push(t);
        k_vec.push(k);
        a_vec.push(a);
    }

    let mut need_vec: Vec<usize> = vec![n];
    let mut mastered: HashSet<usize> = HashSet::<usize>::new();
    while !need_vec.is_empty() {
        let check = need_vec.pop().unwrap();
        if !mastered.contains(&check) {
            mastered.insert(check.clone());
            for need in &a_vec[check - 1] {
                need_vec.push(need.clone());
            }
        }
    }

    let mut ans = 0_usize;
    for i in mastered.iter() {
        ans += t_vec[i - 1];
    }

    println!("{}", ans);
}
