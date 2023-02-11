use proconio::input;

fn main() {
    input! {
        n:usize,
        ab_vec : [ [ i128; 2]; n]
    }
    for ab in ab_vec {
        println!("{}", ab[0] + ab[1]);
    }
}
