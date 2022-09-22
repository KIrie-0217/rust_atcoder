use proconio::input;
use itertools::Itertools;
use std::collections::HashSet;
fn main(){

    input!{
        k:usize ,
        q:[ (usize , usize ) ; k]
    }
    let mut used_row :Vec<usize> = Vec::new();
    let mut used_col :Vec<usize> = Vec::new();
    for ( r ,c ) in q.iter(){
        used_row.push(r.clone() );
        used_col.push(c.clone() );
    }

    let mut col_q :Vec<usize> = Vec::new();
    let mut row_q :Vec<usize> = Vec::new();
    for i in 0usize..8usize{
        if !used_row.contains(&i){
            row_q.push(i.clone() );
        }

        if !used_col.contains(&i){
            col_q.push(i.clone() );
        }
    }

    println!("{:?}", row_q);
    for ( count , i) in col_q.iter().permutations(8-k).enumerate(){
        println!("{:?}" ,  i )
    }
}