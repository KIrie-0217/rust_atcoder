use proconio::input;

fn main(){

    input!{
        n:usize
    }
    if n <= 15 {
        println!( "0{:X}" , n )
    }else{
        println!( "{:X}" , n )
    }
}