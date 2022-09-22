use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,
        xy:[ ( f64 ,f64 ); n]
    }

    let perm = (0..n).permutations(n);
    let mut count:f64 = 0.0;
    let mut dis :f64= 0.0;
    for i in perm{
        // println!("{:?}", i);
        count += 1.0;
        for j in 0..n-1{
            // print!("{:?} " , j);
            let ( x1 , y1 ) = xy[i[j]];
            let ( x2 , y2 ) = xy[i[j+1]];

            let dx = x1 - x2;
            let dy = y1 - y2;
            dis +=  ( dx*dx + dy*dy ).sqrt();
        }
        // println!();
    }
    // println!("{:?}" , count );
    // println!("{:?}", dis);
    let ans = dis / count;
    println!("{}", ans );

}