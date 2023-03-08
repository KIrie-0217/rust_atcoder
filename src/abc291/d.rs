use proconio::input;

fn main() {
    input! {
        n:usize,
        ab_list:[[u64;2];n]
    }

    let mut dp: Vec<Vec<u64>> = vec![vec![0; 2]; n];
    let MOD: u64 = 998_244_353;
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        // dp[i][0]について
        if ab_list[i - 1][0] != ab_list[i][0] {
            dp[i][0] += dp[i - 1][0] % MOD;
        }
        if ab_list[i - 1][1] != ab_list[i][0] {
            dp[i][0] += dp[i - 1][1] % MOD;
        }
        dp[i][0] = dp[i][0] % MOD;

        // dp[i][1]について
        if ab_list[i - 1][0] != ab_list[i][1] {
            dp[i][1] += dp[i - 1][0] % MOD;
        }
        if ab_list[i - 1][1] != ab_list[i][1] {
            dp[i][1] += dp[i - 1][1] % MOD;
        }
        dp[i][1] = dp[i][1] % MOD;
    }

    println!("{}", (dp[n - 1][0] + dp[n - 1][1]) % MOD);
}
