use proconio::input;

fn main() {
    input! {
        n:usize,mut s:usize,
        ab_vec:[[usize;2];n]
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=s {
            if dp[i][j] == 1 {
                if j + ab_vec[i][0] <= s {
                    dp[i + 1][j + ab_vec[i][0]] = 1;
                }
                if j + ab_vec[i][1] <= s {
                    dp[i + 1][j + ab_vec[i][1]] = 1;
                }
            }
        }
    }

    println!("{:?}", dp);
    if dp[n][s] == 1 {
        let mut ans: Vec<String> = vec!['H'.to_string()];
        for i in (1..=n - 1).rev() {
            if (s >= ab_vec[i][0] as usize) && (dp[i][s - ab_vec[i][0] as usize] == 1) {
                ans.push('H'.to_string());
                s -= ab_vec[i][0] as usize;
            } else {
                ans.push('T'.to_string());
                s -= ab_vec[i][1] as usize;
            }
        }
        println!("Yes");
        println!(
            "{}",
            ans.iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    } else {
        println!("No");
    }
}
