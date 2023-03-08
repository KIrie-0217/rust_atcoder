use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,k:usize,
        s:[Chars;n]
    }

    let mut ans: usize = 0;
    for i in k..=n {
        for perm in s.iter().combinations(i) {
            let mut cnt: Vec<usize> = vec![0; 27];
            for v in perm.iter() {
                for j in v.iter() {
                    cnt[(j.clone() as u8 - b'a') as usize] += 1;
                }

                ans = ans.max(
                    cnt.iter()
                        .filter(|x| *x == &k)
                        .collect::<Vec<&usize>>()
                        .len(),
                );
            }
        }
    }

    println!("{}", ans);
}
