use proconio::input;

fn main(){

    input!{
        n:usize,
        a:[ usize; n]
    }

    let mut count:usize = 0;
    let mut ans:usize = 0;

    for i in 0..n{

        if i == a[i] - 1 {

            // println!("aaa");
            count += 1;
        }
        else if i == a[ a[i] -1 ] -1 && i != a[i] -1 {
            ans += 1;
            // println!("{}", ans);
        }
    }
    ans /= 2;
    ans += count*(count -1 )/2;

    println!("{}", ans as usize);
}