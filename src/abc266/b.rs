use proconio::input;

fn main(){

    input!{
        n:i64
    }

    let num:i64 = 998_244_353;
    // let quotient = n/num ;
    let remainder = n%num;

    if remainder <0{
        println!("{}",remainder+num);
    }else{
        println!("{}", remainder);
    }
}