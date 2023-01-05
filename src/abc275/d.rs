use proconio::input;
use std::collections::HashMap;

fn main(){

    input!{
        n:i128
    }
    
    let mut memo = HashMap::new();
    memo.insert(0 as i128,1 as i128);

    fn f(memo:&mut HashMap<i128,i128> ,x:i128) -> i128{
        if let Some( num ) = memo.get( &x ){ // こうするとHashMapへのアクセスが１回で済む
            return *num
        }else{
            let x_1 = f(memo , x/2);
            let x_2 = f(memo , x/3 );

            memo.insert( x/2 , x_1 );
            memo.insert( x/3 , x_2);
            memo.insert( x , x_2 + x_1); 

            return  x_2 + x_1 ;
        }
    }

    println!("{}", f( &mut memo , n));

}