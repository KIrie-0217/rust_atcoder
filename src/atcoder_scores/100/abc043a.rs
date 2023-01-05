use proconio::input;

fn main(){

    input!{
        n:usize
    }
    
    let mut ans:i32 = 0;
    for i in  1..n+1 {
        ans += i as i32;
    }
    
    println!("{}" , ans );
}