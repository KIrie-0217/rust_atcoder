use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars
    }

    let mut split: Vec<u128> = vec![1];

    // 足し算ごとに数式を分割する
    // 掛け算の結果は0か0でないか、だけが重要
    for c in s.iter() {
        match c {
            '+' => split.push(1),
            '0' => *split.last_mut().unwrap() *= 0,
            _ => continue,
        }
    }

    let mut ans: usize = 0;
    for tip in split {
        if tip != 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
