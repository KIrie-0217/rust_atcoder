use proconio::input;

fn main(){
    input!{
        a:i64,
        b:i64,
        c:i64,
        x:i64,
        y:i64,
    }

    let mut max_piza: i64 = x.max(y);
    max_piza *= 2;
    let mut price = std::i64::MAX;
    // println!("{}", price);
    for i in 0..max_piza +1{
        let mut tmp: i64= 0;
        tmp += c  * i ;
        tmp += a  * 0.max( x - i/2 );
        tmp += b  * 0.max( y - i/2 );

        price = price.min( tmp );
    }

    println!("{}", price as i64);
}