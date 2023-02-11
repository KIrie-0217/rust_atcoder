use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[i64;n]
    }

    let mut a_j3: Vec<i64> = Vec::<i64>::new();
    let mut a_j2: Vec<i64> = Vec::<i64>::new();
    let mut a_j1: Vec<i64> = Vec::<i64>::new();

    a_j3.push(a[0].clone().pow(2));
    let mut aj2_sum: i64 = 0;
    for i in 1..n {
        if i != n - 1 {
            a_j3.push(a[i].clone().pow(2) + a_j3[i - 1]);
        }

        aj2_sum += a[i - 1].clone();
        a_j2.push(a[i].clone() * aj2_sum);

        a_j1.push(a[i].clone().pow(2) * i as i64);
    }

    let mut ans: i64 = 0;
    ans += a_j1.iter().sum::<i64>();
    ans -= 2_i64 * a_j2.iter().sum::<i64>();
    ans += a_j3.iter().sum::<i64>();

    println!("{}", ans);
}
