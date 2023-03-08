use proconio::input;
use std::collections::{HashMap, VecDeque};

fn main() {
    input! {
        n:usize,m:usize,
        mut xy_list:[[usize;2];m]
    }

    let mut a_front: HashMap<usize, Vec<usize>> = HashMap::<usize, Vec<usize>>::new();
    let mut a_prior: HashMap<usize, usize> = HashMap::<usize, usize>::new();

    for xy in xy_list {
        let x = xy[0];
        let y = xy[1];

        a_front.entry(x).or_insert(vec![]).push(y);
        *a_prior.entry(y).or_insert(0) += 1;
    }

    let mut ans: Vec<usize> = vec![];
    let mut queue: VecDeque<usize> = VecDeque::<usize>::new();
    let mut count :usize= 1;
    for i in 0..n{
        if !a_prior.contains_key(i){
            queue.push_back(i);
        }
    }
    while( !queue.is_empty() ){
        if( queue.len() > 1){
            println!("No");
            return ;
        }
        let current = queue.pop().unwrap();
        ans[current] = count;
        count += 1;
    }
}
