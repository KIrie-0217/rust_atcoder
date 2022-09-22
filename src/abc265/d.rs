use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn binary_search( input_hash:&HashMap<i64,i64> , min:i64 , ans:i64 ) -> i64{
    let mut min_index = min -1 ;
    let mut max_index = input_hash.len() as i64 ;
    while max_index - min_index > 1{
        let mid_index = ( min_index + max_index )/2 ;
        if input_hash[ &mid_index ] > ans{
            max_index = mid_index as i64;
        }else{
            min_index = mid_index as i64;
        }

    }

    max_index
}

fn main(){

    input!{
        n:usize, p:i64, q:i64 , r:i64,
        a : [ i64 ; n]
    }

    let mut is_exist = false;
    let mut sum_hash = HashMap::new();
    let mut tmp:i64 = 0;


    for (count ,i ) in a.iter().enumerate(){
        tmp += i;
        sum_hash.insert(count as i64 , tmp.clone());
    }

    sum_hash.insert( -1 , -1);
    sum_hash.insert( n as i64 , sum_hash[ &( n as i64  -1 ) ] + 1000 );

    // println!("{:?}",sum_hash);


    for x in 0..n{

        // println!("{}", x);
        let sy = sum_hash[&(x as i64) ] + p;
        let sy_index = binary_search(&sum_hash, x as i64, sy);
        if sum_hash[ &sy_index ] != sy{
            continue
        }else{
            let sz = sum_hash[ &sy_index ] + q;
            let sz_index = binary_search(&sum_hash, sy_index, sz);
            if sum_hash[ &sz_index ] != sz{
                continue
            }else{
                let sw = sum_hash[ &sz_index ] + r;
                let sw_index = binary_search(&sum_hash, sz_index, sw);
                if sum_hash[ &sw_index ] != sw{
                    continue
                }else{
                    is_exist = true;
                }
            }
        }
    }

    if is_exist{
        println!("Yes");
    }else{
        println!("No");
    }

}