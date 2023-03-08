use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n]
    }

    let mut total_xor = 0_usize;
    for ai in &a {
        total_xor ^= ai;
    }

    let mut ans: Vec<usize> = vec![];
    for ai in &a {
        ans.push(total_xor ^ ai);
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
