use proconio::input;

fn main(){

    input!{
        s:String
    }

    let s_len = s.len();

    println!("{}", &s[(s_len/2)..(s_len/2+1)]);
}

