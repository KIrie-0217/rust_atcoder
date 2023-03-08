use proconio::input;
use std::{collections::HashMap, i64::MIN};

fn main() {
    input! {
        n:usize,
        txa:[[usize;3];n]
    }

    let minimum = MIN;
    let mut tx_map: HashMap<usize, Vec<usize>> = HashMap::<usize, Vec<usize>>::new();
    for i in 0..n {
        tx_map.insert(txa[i][0], vec![txa[i][1], txa[i][2]]);
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 5]; txa[n - 1][0] + 1];
    for i in 1..5 {
        dp[0][i] = minimum;
    }
    for t in 1..=txa[n - 1][0] {
        for x in 0..5 {
            let mut before_max: i64 = dp[t - 1][x];
            if x != 0 {
                before_max = before_max.max(dp[t - 1][x - 1]);
            }
            if x != 4 {
                before_max = before_max.max(dp[t - 1][x + 1]);
            }
            dp[t][x] += before_max;

            match tx_map.get_mut(&t) {
                Some(xa) => {
                    if xa[0] == x {
                        dp[t][x] += xa[1] as i64;
                    }
                }
                None => continue,
            }
        }
    }

    let mut ans: i64 = 0;
    for i in 0..5 {
        ans = ans.max(dp[txa[n - 1][0]][i]);
    }

    println!("{}", ans);
}
