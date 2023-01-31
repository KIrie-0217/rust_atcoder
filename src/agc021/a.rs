use proconio::input;

fn main() {
    input! {
        n:u128
    }
    let num_vec: Vec<u32> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut ans: u32 = 0;
    for (i, c) in num_vec.iter().enumerate() {
        if i == 0 {
            ans += c - 1;
        } else {
            ans += 9;
        }
    }
    println!("{}", ans.max(num_vec.iter().sum()));
}
