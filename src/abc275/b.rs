use proconio::input;

fn main(){

    input!{
        a:i128,
        b:i128,
        c:i128,
        d:i128,
        e:i128,
        f:i128
    }
    
    let warukazu = 998244353 as i128;
    let left:i128 = a%warukazu * b%warukazu * c%warukazu;
    let right:i128 = d%warukazu * e%warukazu * f%warukazu;

    let mut ans:i128 = (left - right)%warukazu;

    if ans < 0{
        ans += warukazu;
    }
    println!("{}", ans);
}