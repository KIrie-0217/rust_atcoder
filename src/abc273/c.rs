use itertools::Itertools;
use proconio::input;
use std::collections::BTreeMap;

fn main(){

    input!{
        n:usize,
        mut a : [ usize ; n]
    }

    let mut count_bt = BTreeMap::new();
    for i in a.iter(){
        
        *count_bt.entry(i).or_insert(0 as usize ) += 1;

    }

    let mut res = count_bt.iter().rev()
                        .map( |node| *node.1 ).collect_vec();
    while res.len() < n{
        res.push(0);
    }

    for i in res.iter(){
        println!("{}", i);
    }

}
