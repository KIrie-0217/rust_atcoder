use proconio::input;

fn main() {
    input! {
        x:u128,y:u128
    }

    let mut count: usize = 1;
    let mut a_i: u128 = x;
    loop {
        a_i *= 2;
        if a_i > y {
            break;
        }
        count += 1;
    }

    println!("{}", count);
}
