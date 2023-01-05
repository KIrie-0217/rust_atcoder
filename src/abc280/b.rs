use proconio::input;

fn main(){

    input!{
        n:usize,
        seq : [ i64 ; n]
    }
   
    let mut res : Vec<i64> = Vec::new();
    let mut cum : i64 = 0;
    for s in seq{
        if res.len() == 0 {
            res.push( s );
            cum = s
        }else{
            res.push( s - cum );
            cum += s -cum;
        }
    }

    let dst : Vec<String> = res.iter().map( |x| x.to_string() ).collect::<Vec<String>>();
    println!("{}", dst.join(" "))
}