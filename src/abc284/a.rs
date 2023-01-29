use proconio::input;

fn main() {
    input! {
        n:usize,
        input_strs:[ String; n]
    }

    for i in 0..n {
        let rev = n - 1 - i;
        println!("{}", &input_strs[rev]);
    }
}
