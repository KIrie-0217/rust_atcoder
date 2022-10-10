use proconio::input;

fn main(){

    input!{
        n:usize , m:usize,
    }

    let mut counter  = vec![ vec![ 0; n] ; n];
    for i in 0..n{
        counter[i][i] = 1;
    }
    
    for i in 0..m{
        input!{
            k:usize , x: [ usize ; k ]
        }
        for &val1 in &x{
            for &val2 in &x{
                counter[ val1 -1][ val2 -1] += 1 ;
                counter[ val2 -1][ val1 -1] += 1 ;
            }
        }
    }

    for i in 0..n{
        for j in counter[i].iter(){
            if j == &0{
                println!("No");
                return
            }
        }
    }

    println!("Yes");
    return
}