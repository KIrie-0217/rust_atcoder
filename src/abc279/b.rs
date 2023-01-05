use itertools::Itertools;
use proconio::input;

fn main(){

    input!{
        s:String, t:String
    }

    /* 
    let res : Vec<&str> = s.matches(&t).collect();
    // let res : Vec<_> = s.match_indicies(&t).collect();
    if res.len() == 0 {
        println!("No");
    }else{
        println!("Yes");
    }
    */

    let res = s.find(&t); 
    match res {
        Some(_) => {println!("Yes")},
        None => {println!("No")},
    }
}