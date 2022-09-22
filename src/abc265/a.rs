use::proconio::input;

fn main(){

    input!{
        x:usize, y:usize, n:usize
    }

    if x*3 > y{
        let apple_set = n / 3;
        let apple_single = n - apple_set*3;

        println!("{}", apple_set*y + apple_single*x);
    }else{
        println!("{}", x*n);
    }
}