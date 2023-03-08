use proconio::input;

fn can_reach_w2(r1: i64, c1: i64, r2: i64, c2: i64) -> bool {
    return ((r1 - r2).abs() + (c1 - c2).abs() <= 6_i64)
        | ((r1 + c1 + r2 + c2) % 2 == 0)
        | (((r1 + c1) - (r2 + c2)).abs() <= 3_i64)
        | (((r1 - c1) - (r2 - c2)).abs() <= 3_i64);
}

fn main() {
    input! {
        r1:i64,c1:i64,
        r2:i64,c2:i64
    }

    let mut ans: usize = 0;
    if (r1 == r2) && (c1 == c2) {
        ans = 0;
    } else if (r1 + c1 == r2 + c2)
        | (r1 - c1 == r2 - c2)
        | ((r1 - r2).abs() + (c1 - c2).abs() <= 3_i64)
    {
        ans = 1;
    } else if can_reach_w2(r1, c1, r2, c2) {
        ans = 2;
    } else {
        ans = 3;
    }

    println!("{}", ans);
}
