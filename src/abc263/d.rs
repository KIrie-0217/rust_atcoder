use proconio::input;


fn main(){
    
    input!{
        n:i64,
        l:i64,
        r:i64,
        a:[ i64 ; n]
    }

    let a_sum : i64 = a.iter().sum();
    let mut l_vec = vec![ 0 ; n as usize + 2];
    let mut r_vec = vec![ 0 ; n as usize + 2];
    let mut l_cur = 0;
    let mut r_cur = 0;

    for i in 0..n{

        l_cur += a[i as usize] - l;
        r_cur += a[ ( n as usize  - 1) -i as usize ] - r;

        l_vec[ i as usize + 1  ] = l_vec[i as usize].max( l_cur );
        r_vec[ n as usize   - i as usize  ] = r_vec[ n  as usize - i as usize + 1  ].max( r_cur );


    }

    let mut res = 0;
    for i in 0..n+1{
        res = res.max( l_vec[i as usize] + r_vec[ i as usize + 1] );

    }

    println!("{}", a_sum - res );

}
