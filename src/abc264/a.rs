use proconio::input;

fn main(){

    input!{
        l:usize,
        r:usize
    }

    let atcoder ="atcoder";
    let mut ans = "".to_string();
    for ( i , c ) in atcoder.chars().enumerate(){
        if  i >= l -1  && i < r{
            ans.push(c);
        }
    }

    println!("{}" ,ans);
}