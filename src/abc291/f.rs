use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n:usize,m:usize,
        s_list:[Chars;n]
    }

    let mut can_tele_list: Vec<Vec<usize>> = vec![vec![]; n];
    for (i, c_vec) in s_list.iter().enumerate() {
        for (j, c) in c_vec.iter().enumerate() {
            if *c == '1' {
                can_tele_list[i].push(j + i + 1);
            }
        }
    }

    println!("{:?}", can_tele_list);
    let mut arrived : Vec<bool> = vec![false;n];
    
}
