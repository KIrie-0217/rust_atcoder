use proconio::input;
use itertools::Itertools;


fn main(){

    input!{
        h1:usize,
        w1:usize
    }
    let mut a = Vec::new();
    for _ in 0..h1{
        input!{
            a_tmp:[i64 ; w1]
        }
        a.push(a_tmp);
    }
    input!{
        h2:usize,
        w2:usize
    }
    let mut b = Vec::new();
    for _ in 0..h2{
        input!{
            b_tmp:[i64 ; w2]
        }
        b.push(b_tmp);
    }

    fn check( raw_comb:Vec<usize> , col_comb:Vec<usize> , a:Vec<Vec<i64>> , b:Vec<Vec<i64>> ) -> bool{
        for (i ,raw) in raw_comb.iter().enumerate(){
            for (j ,col) in col_comb.iter().enumerate(){
                if a[*raw as usize][*col as usize] != b[i][j]{
                    return false
                }
            }
        }
        true
    }

    let mut isMatch = false;
    for raw_comb in (0..h1).combinations(h2){
        if !isMatch{
            for col_comb in (0..w1).combinations(w2){
                if !isMatch{
                    let raw_comb_tmp = &raw_comb;
                    let a_tmp = &a;
                    let b_tmp = &b;
                    if check( raw_comb_tmp.to_vec(), col_comb, a_tmp.to_vec(), b_tmp.to_vec()){
                        isMatch = true;
                        break;
                    }
                }
            }
        }
    }
    if isMatch{
        println!("Yes");
    }else{
        println!("No");
    }

}