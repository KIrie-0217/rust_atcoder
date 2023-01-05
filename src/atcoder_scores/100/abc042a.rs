use proconio::input;
use std::collections::HashMap;

fn main(){

    input!{
        a: usize,
        b: usize,
        c: usize
    }
    let mojis = vec![a,b,c];
    let mut h_map = HashMap::new();
    for i in mojis.iter(){
        *h_map.entry(*i as i32).or_insert(0) += 1;
    }

    if h_map.contains_key( &5 ){
        if h_map[ &5 ] == 2{
            if h_map.contains_key( &7 ){
                if h_map[ &7 ] == 1{
                    println!("YES");
                    return 
                }
            }
        }
    }
    
    println!("NO");
    
    return
}