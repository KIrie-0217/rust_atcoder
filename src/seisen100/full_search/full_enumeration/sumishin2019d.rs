use proconio::input;

fn main(){

    input!{
        n:usize,
        s:String,
    }

    let mut ans = 0;
    for i in 0..1000{
        let mut d = 0;
        let i_str : Vec<char> = format!("{:03}",i).chars().collect(); // string -> chars -> vec
        for c in s.chars(){
            if i_str[d] == c{ d += 1; }
            if d == 3{
                ans += 1;
                break;
              }
        }

    }

    println!("{}", ans);


}