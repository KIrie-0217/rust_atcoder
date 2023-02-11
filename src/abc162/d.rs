use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars
    }
    let r_count: usize = s.iter().filter(|&c| *c == 'R').count();
    let g_count: usize = s.iter().filter(|&c| *c == 'G').count();
    let b_count: usize = s.iter().filter(|&c| *c == 'B').count();

    let ans: usize = r_count * g_count * b_count;
    let mut same_distanse_count: usize = 0;
    for i in 0..n {
        for j in i..n {
            let k = 2 * j - i; // j-i = k-j
            if k >= n {
                break;
            }
            if s[i] != s[j] && s[j] != s[k] && s[i] != s[k] {
                same_distanse_count += 1;
            }
        }
    }

    println!("{}", ans - same_distanse_count);
}
