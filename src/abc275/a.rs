use proconio::input;

fn main(){

    input!{
        n:usize,
        h:[ i32; n ]
    }

    let mut max_num = -1 as i32;
    let mut res = 1;
    for i in 0..n{
        
        if h[i] > max_num{
            res = i + 1;
            max_num = h[i];

        }
    }

    println!("{}", res);
}