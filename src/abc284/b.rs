use proconio::input;

fn main() {
    input! {
        t:usize
    }

    for _ in 0..t {
        input! {
            n:usize,
            a_arr:[ usize; n]
        }

        let mut count: usize = 0;
        for a_i in a_arr {
            if a_i % 2 != 0 {
                count += 1;
            }
        }
        println!("{}", count);
    }
}
