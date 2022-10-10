use proconio::input;

fn main(){

    input!{
        n:usize
    }
    if n <= 15 {
        println!( "0{:0X}" , n )
    }else{
        println!( "{:0X}" , n )
    }
}