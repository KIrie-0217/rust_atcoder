use proconio::input;

fn main(){

    input!{
        n:usize, 
        s:[ usize; n],
        q:usize,
        t:[ usize; q]
    }
    
    let mut count = 0;
    for i in t.iter(){

        let mut top = 0usize;
        let mut end = n-1;
        while end - top > 1{
            let mid:usize = ( end + top )/2;
            if s[ mid ] >= *i{
                end =mid;
            }else{
                top = mid;
            }
        }
        if s[end] == *i || s[top] == *i{
            count += 1;
        }
    }

    println!("{}", count);
}