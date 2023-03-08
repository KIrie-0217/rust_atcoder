use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n]
    }
    if n == 0 {
        println!("{}", a[0]);
        return;
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0_i64; 2]; n];
    const MOD: i64 = 1_000_000_007;

    dp[0][0] = (a[0] + a[1]).rem_euclid(MOD);
    dp[0][1] = (a[0] - a[1]).rem_euclid(MOD);

    for i in 1..n - 1 {
        dp[i][0] = (dp[i - 1][0] + dp[i][1] + x * a[i + 1]).rem_euclid(MOD);
        dp[i][1] = (dp[i-1][0] - ) 
    }
}
