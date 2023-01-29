use proconio::input;

fn main() {
    input! {
        n:usize,
        k:i64,
        mut a_vec :[i64;n],
        mut b_vec :[i64;n]
    }

    let mut dp: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    dp.push(vec![true, true]);

    for i in 0..n - 1 {
        dp.push(vec![false, false]);
        if dp[i][0] {
            if (a_vec[i] - a_vec[i + 1]).abs() <= k {
                dp[i + 1][0] = true;
            }
            if (a_vec[i] - b_vec[i + 1]).abs() <= k {
                dp[i + 1][1] = true;
            }
        }
        if dp[i][1] {
            if (b_vec[i] - a_vec[i + 1]).abs() <= k {
                dp[i + 1][0] = true;
            }
            if (b_vec[i] - b_vec[i + 1]).abs() <= k {
                dp[i + 1][1] = true;
            }
        }
    }

    if dp[n - 1][0] | dp[n - 1][1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
