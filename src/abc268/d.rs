use proconio::input;
use itertools::Itertools;


fn dfs(insert:i32 ,padding:i32 ,index:&Vec<usize>, s:&Vec<String>, t:&Vec<String>) -> (String,bool){

    if insert > padding{
        return ( "".to_string() , false);
    }
}
fn main(){

    input!{
        n:usize, m:usize,
        s: [ String; n],
        t: [ String; m]
    }

    let mut count = 0;
    for i in s.iter(){
        count += i.len() + 1;
    }
    count -= 1;
    let check = count <= 16;
    // println!("{}",check);
    let padding = 16 - count;
    // println!( "{}", padding);
    if check{
        let perm = (0 .. n ).permutations(n);
        for i in perm{
            let mut ans_tmp = String::new();
            for j in i.iter(){
                ans_tmp = ( ans_tmp + &s[ *j ] ).to_string();
                ans_tmp += "_";
            }
            ans_tmp = ans_tmp[0..ans_tmp.len()-1].to_string();
            if !t.contains(&ans_tmp) { 
                println!("{}" , ans_tmp);
                return
            }
        }
    }

    println!("{}", -1);
    return 
}