use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars
    }

    let mut ans: Vec<usize> = Vec::<usize>::new();
    for c in s {
        match c {
            '0' => ans.push(1),
            '1' => ans.push(0),
            _ => unreachable!(),
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
