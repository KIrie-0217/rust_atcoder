use proconio::input;

fn main(){

    input!{
       a:f32,
       b:f32 
    }
    


    let ans:f32 = b/a;

    println!("{:.3}", ( ans * 1000.0).round()/1000.0 );
    
    return 

}