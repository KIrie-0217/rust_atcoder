use proconio::input;

fn main(){

    input!{
        n:usize , q:usize

    }

    let mut target_array = Vec::new();
    for i in 0..n{

        input!{
            l:usize,
            a: [ usize ; l ]
        }
        
        target_array.push( a );
        
    }
    
    for _ in 0..q{

        input!{
            s:usize,
            t:usize
        }
    
       println!("{}", target_array[s-1][t-1]); 
    }
}