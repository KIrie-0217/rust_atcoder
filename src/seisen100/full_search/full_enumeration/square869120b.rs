use proconio::input;


fn median( a : &Vec<i64>) -> i64{
    let mut sort_a = a.clone();
    sort_a.sort();
    let len = sort_a.len();
    let target = len/2 ;
    
    let mut med = if len %2 == 0 {
        (( sort_a[target]  sort_a[target -1] ) as f64 / 2.0 ) as i64
    }else{
        sort_a[target]
    };
    med
}

fn main(){
    input!{

        n : usize,
        ab : [ (i64, i64) ; n]
    }
    let mut A : Vec<i64> = vec![];
    let mut B : Vec<i64> = vec![];
    for ( a , b ) in ab.iter(){
        A.push(*a);
        B.push(*b);
    }
    let med_a = median( &A );
    let med_b = median( &B );
    let mut ans = 0;
    for ( a , b ) in ab.iter(){
        ans += ( a - med_a).abs() + ( b - a) + ( b - med_b).abs();
    }

    println!("{}", ans);
}