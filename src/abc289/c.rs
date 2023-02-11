use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,m:usize
    }
    let mut cs: Vec<u16> = vec![];
    for _ in 0..m {
        cs.push(0);
        input! {
            c: usize,
            aic: [usize; c],
        }

        for i in aic.iter() {
            if let Some(last) = cs.last_mut() {
                *last += 1_u16 << i - 1;
            }
        }
    }

    // 集合の中からm個選ぶ組み合わせを考える
    let mut ans: usize = 0;
    for i in 1..=m {
        for comb in (0..m).combinations(i) {
            let mut check: u16 = 0;
            for p in comb {
                check |= cs[p];
            }

            if check == 2_u16.pow(n as u32) - 1 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
