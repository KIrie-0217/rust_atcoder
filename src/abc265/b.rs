use proconio::input;
use std::collections::HashMap;

fn main(){

    input!{
        n:usize, m:usize, t:i64,
        a:[ i64; n-1],
        xy:[ (usize , i64) ; m]
    }

    let mut bonus_time = HashMap::new();
    for (x,y) in xy{
        bonus_time.insert(x,y);
    }

    let mut time_left:i64 = t;
    let mut can_clear : bool = true;
    for (count , time) in a.iter().enumerate(){
        time_left -= time;

        if time_left <= 0{
            can_clear = false;
        }

        if bonus_time.contains_key(&(count + 2usize) ){
            time_left += bonus_time[&(count + 2usize) ]
        }
    }

    if can_clear{
        println!("Yes");
    }else{
        println!("No");
    }

}