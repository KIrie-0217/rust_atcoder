use proconio::input;

fn main(){

    input!{
        n:usize,
        a:[ usize ; n ]
    }
    
    let mut odd = Vec::new();
    let mut even = Vec::new();

    for i in a.iter(){
        if i%2 == 0{
            even.push( i );
        }else{
            odd.push( i );
        }
    }
    even.sort_by( | a , b | b.cmp( a ) );
    odd.sort_by( | a , b | b.cmp( a ) );
    
    let mut res:usize = 0 ;
    
    if even.len() >= 2{
        let tmp = even[0] + even[1];
        res = std::cmp::max( tmp , res );
    }

    if odd.len() >= 2 {
        let tmp = odd[0] + odd[1];
        res = std::cmp::max( tmp , res );
    }

    if res == 0{
        println!("{}", -1 );
    }else{
        println!("{}", res);
    }
    
}