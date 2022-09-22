use proconio::input;

fn main(){

    input!{
        n:usize , m:usize,
        a: [ i64; n]
    }

    let mut csum:Vec<i64>= Vec::new();
    let mut bsum:Vec<i64>= Vec::new();
    let mut tmp:i64 = 0;
    for i in a.iter(){
        tmp += i;
        csum.push( tmp.clone() );
    }


    let mut bsum_tmp:i64 = 0;
    for i in 0..m{
        bsum_tmp += a[i as usize ] * (i + 1) as i64;
    }

    bsum.push( bsum_tmp.clone() );
    for i in (m -1 )..n-1{
        if i == m-1{
            bsum_tmp -= csum[i];
        }else{
            bsum_tmp -=  csum[i]  - csum[i-m]  ;
        }
        //println!("{}", a[i as usize + m -1 ] );
        bsum_tmp += a[i as usize + 1]*m as i64;

        bsum.push( bsum_tmp.clone() );
    }

    let mut ans:i64 = std::i64::MIN;
    for i in bsum{
        ans = ans.max(i);
    }

    println!("{}", ans);
}