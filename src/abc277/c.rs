use proconio::input;
use std::collections::{HashMap , HashSet};

fn main(){

    input!{
        n:usize,
        ab: [ [usize; 2] ; n]
    }
    let mut ladder:HashMap<usize,Vec<usize> > = HashMap::new();
    for i in ab.iter(){
        ladder.entry(i[0]).or_insert(vec![]).push(i[1]);
        ladder.entry(i[1]).or_insert(vec![]).push(i[0]);
    }    

    if !ladder.contains_key( &(1 as usize ) ){
        println!("1");
        return
    }

    let mut arrival:HashSet<usize> = vec![1 as usize].into_iter().collect();
    let mut que = vec![1 as usize];
    
    while que.len() != 0{
        let v = que.pop().unwrap();
        for i in &ladder[ &v ]{
            if !arrival.contains( &i ){
                que.push( i.clone() );
                arrival.insert(i.clone());
            }
        }
    }

    println!("{:?}", arrival.iter().max_by(|a, b| a.cmp(&b)).unwrap() );
}