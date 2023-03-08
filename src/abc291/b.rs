use proconio::input;

fn main() {
    input! {
        n:usize,
        mut x:[f64;n*5]
    }

    x.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut ans: f64 = 0_f64;
    for i in n..4 * n {
        ans += x[i];
    }

    println!("{}", ans / (3_f64 * n as f64));
}
