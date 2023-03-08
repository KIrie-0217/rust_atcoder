use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s:Chars,
    }

    for (i, c) in s.iter().enumerate() {
        if c.is_uppercase() {
            println!("{}", i + 1);
            return;
        }
    }
}
