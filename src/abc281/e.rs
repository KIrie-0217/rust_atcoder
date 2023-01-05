use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,
        m:usize,
        k:usize,
        a: [ usize; n]
    }

    let a_part:&[usize] = &a[0..m];
    let mut vec_ans:Vec<usize> = Vec::new();
    println!("{:?}", a_part);
    let left = a_part[0];
    let sort_next:usize = *a_part.iter().sorted().as_slice()[k] ;

    let mut ans_tmp :usize =  a[0..k].iter().sum() ;
    vec_ans.push( ans_tmp.clone() );

    for i in  1 as usize..( n-m+1 ) as usize{

        let next = a[i + m -1 ]; 

        if left < sort_next{
            ans_tmp -= left;
        }
        
        if next < sort_next{
            ans_tmp += next;
        }
        
    }

}