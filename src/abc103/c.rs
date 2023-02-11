use proconio::input;

fn main() {
    input! {
        n:usize,
        a_arr:[usize;n]
    }
    let mut ans: usize = 0;
    for ai in a_arr {
        ans += ai - 1_usize;
    }
    println!("{}", ans);
}
