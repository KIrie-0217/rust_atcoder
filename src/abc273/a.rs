use proconio::input;

fn main(){

    input!{
        n:usize
    }

    let ans = f(n);

    println!("{}",ans);
}

fn f(x:usize) -> usize {
    if x == 0{ return 1 }
    else{
        return x*f(x-1 as usize);
    }
}