use proconio::input;

fn main(){

    input!{
        h:usize,
        _w:usize,
        s : [ String ; h ]
    }

    let mut res :usize = 0;
    for col in s{
        let tmp: &str = &col;
        res += tmp.match_indices("#").count();
    } 

    println!("{}", res);
}