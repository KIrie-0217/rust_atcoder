use proconio::input;
use std::{
    collections::{HashSet, VecDeque},
    iter::FromIterator,
};

// tle
fn dfs(now: usize, a: &HashSet<usize>, b: &HashSet<usize>, x: usize) -> bool {
    if b.contains(&now) {
        return false;
    }
    if now == x {
        return true;
    }
    if now > x {
        return false;
    }

    for i in a.iter() {
        if dfs(now + i, a, b, x) {
            return true;
        }
    }

    return false;
}

// tle
fn bfs(a: &Vec<usize>, b: &HashSet<usize>, x: usize) -> bool {
    let mut queue: VecDeque<usize> = VecDeque::<usize>::new();
    let start: usize = 0_usize;
    queue.push_back(start);

    loop {
        if queue.is_empty() {
            break;
        }

        let now = queue.pop_front().unwrap();

        for i in a {
            let next = now + i;
            if next == x {
                return true;
            }

            if next < x && !b.contains(&(next)) {
                queue.push_back(next);
            }
        }
    }

    return false;
}

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        m:usize,
        b:[usize;m],
        x:usize
    }

    let mut dp: Vec<bool> = vec![false; x + 1];
    let mut not_mochi: Vec<bool> = vec![true; x + 1];
    for i in b {
        not_mochi[i] = false;
    }

    dp[0] = true;

    for i in 0..=x {
        if not_mochi[i] {
            for j in a.iter() {
                if i >= *j {
                    dp[i] |= dp[i - j];
                }
            }
        }
    }

    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
