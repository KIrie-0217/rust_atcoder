use proconio::input;
fn main(){

    input!{
        k:u8
    }

    for i in 0..k{
        let c = (b'A' + i) as char;
        print!("{}",c);
    } 
}