use proconio::input;

fn main() {
    input! {
        t:usize
    }
    for _ in 0..t {
        input! {
            n:usize,
            _s:[String;3]
        }

        println!(
            "{}{}0",
            (0..n)
                .map(|_x| 0.to_string())
                .collect::<Vec<String>>()
                .join(""),
            (0..n)
                .map(|_x| 1.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
