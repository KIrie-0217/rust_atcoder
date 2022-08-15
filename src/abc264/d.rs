use proconio::input;
use std::collections::HashMap;
use std::collections::VecDeque;

fn swap(current:String , index:usize) -> String{
    let mut res :String = String::new();
    let current :String = current.to_string();
    let mut tmp:char='_';

    for (i,c) in current.chars().enumerate(){
        if i == index -1 {
            tmp = c ;
        }else if i == index{
            res.push( c );
            res.push( tmp);
        }else{
            res.push( c );
        }
    }
    res
}

fn main(){

    input!{
        atcoder:String
    }
    let atcoder = atcoder;
    let mut map = HashMap::new();
    map.insert(atcoder.clone(),0);

    let mut queue = VecDeque::new();
    queue.push_back( atcoder );

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current == "atcoder"{
            println!("{}",map[&current]);
            return;
        }

        for i in 1..7{
            let next = current.clone();
            let next = swap( next , i);
            if !map.contains_key( &next ){
                map.insert( next.clone() , map[&current]+1 );
                queue.push_back( next.clone() );

            }
        }

    }

}