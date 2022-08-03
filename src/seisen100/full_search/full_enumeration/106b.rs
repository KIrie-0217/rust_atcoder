use proconio::input;

use math_tools::*;

pub mod math_tools {
    use num::Integer;

    pub fn divisor(n: i64) -> Vec<i64> {
        assert!(n >= 1);
        let mut retval: Vec<i64> = Vec::new();
        for i in 1..=num_integer::sqrt(n) {
            if n.is_multiple_of(&i) {
                retval.push(i);
                if i * i != n {
                    retval.push(n / i);
                }
            }
        }
        retval.sort();
        retval
    }
}


fn main(){

    input!{
        n:usize,
    }

    let mut ans:usize = 0;
    for i in (1..n+1).step_by(2){
        let val = divisor(i as i64);
        if val.len() == 8{
            ans += 1;
        }
    }
    println!("{}", ans);

}