use proconio::input;

fn main(){

    input!{
        n:i64,
        a:[ i64; n],
        q:i64,
        m:[ i64;q]
    }

    for i in m.iter(){
        let mut can_create = false;
        for bit in 0..( 1 << n ){
            let mut count:i64= 0;
            let sub_list = ( 0..n)
                .filter( |x| ( bit & ( 1 << x)) != 0)
                .map( |x| a[ x as usize ] );
            for j in sub_list {
                count += j;
            }
            if count == *i{
                can_create = true;
                break;
            }
        }
        if can_create{
            println!("yes");
        }else{
            println!("no");
        }
    }
}