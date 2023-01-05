use proconio::input;

fn main(){

    input!{
        mut x:i64,k:u32
    }

   
    let mut pow = 10i64;
    for _ in 0..k{
        x = ( ( x + ( pow /2 ) ) / pow ) * pow ;
        pow *= 10;
    }

    println!("{}" , x );
}