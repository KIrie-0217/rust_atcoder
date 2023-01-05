use proconio::input;

fn main(){

    input!{
        s : String
    }

    let count_v = s.match_indices("v").count();
    let count_w = s.match_indices("w").count() * 2 as usize;

    println!("{}", count_v + count_w);
}