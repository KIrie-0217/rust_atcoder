use proconio::input;
use std::collections::HashSet;

fn main(){

    input!{
        n:usize,
        s_vec: [ String ; n]
    }

    let mut s_hash: HashSet<String> = HashSet::new();
    let first_hash: HashSet<char> = vec![ 'H', 'D' , 'C' , 'S'].into_iter().collect();
    let second_hash: HashSet<char> = vec![ 'A' , '2' , '3' , '4' , '5' , '6' , '7' , '8' , '9' , 'T' , 'J' , 'Q' , 'K' ].into_iter().collect();
    for i in s_vec.iter(){
        s_hash.insert( i.clone() );
        if !first_hash.contains( &i.chars().nth(0).unwrap() ){
            println!("No");
            return
        }

        if !second_hash.contains( &i.chars().nth(1).unwrap() ){
            println!("No");
            return
        }
    }
    
    if s_hash.len() != n{
        println!("No");
        return
    };

    println!("Yes");
}