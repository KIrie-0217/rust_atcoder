use proconio::input;

fn main(){

    input!{
        n:usize,
        a:[ usize ; n ]
    }

    let mut res : usize = 0; 
    for i in a.iter(){
       res += i  ;
    }

    println!("{}" , res );
}