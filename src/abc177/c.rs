use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[u128;n]
    }

    let mut sum_vec: Vec<u128> = Vec::<u128>::new();
    sum_vec.push(a[n - 1].clone());
    for i in 1..n - 1 {
        sum_vec.push(sum_vec[i - 1].clone() + a[n - 1 - i].clone());
    }
    sum_vec.reverse();

    let mut ans: u128 = 0;
    for i in 0..n - 1 {
        ans += (a[i] * sum_vec[i]) % (10_u128.pow(9) + 7);
    }

    println!("{}", ans % (10_u128.pow(9) + 7));
}
