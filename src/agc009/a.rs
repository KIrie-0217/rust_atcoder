use proconio::input;

fn main() {
    input! {
        n:usize,
        mut ab:[[usize;2];n]
    }

    let mut ab_surplus: Vec<usize> = vec![0_usize; n + 1];
    for i in (0..=(n - 1)).rev() {
        ab[i][0] += ab_surplus[i + 1];
        let surplus = ab[i][0] % ab[i][1];
        let mut add_count = 0_usize;
        if surplus != 0 {
            add_count = ab[i][1] - surplus;
        }
        ab_surplus[i] = ab_surplus[i + 1] + add_count;
    }

    println!("{}", ab_surplus[0]);
}
