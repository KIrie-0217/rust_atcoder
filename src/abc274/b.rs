use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
       h:usize,
       w:usize
    }

    let mut ans : Vec<usize> =  vec![ 0; w];
    for _i  in 0..h  {
        
        input!{
            c:String
        }


        for (j, pos) in c.as_str().chars().enumerate(){
            
            if pos == '#'{
                ans[j] += 1;
            }else if  pos == '.' {
                ans[j] += 0;
            }
        }
    }

    let res = ans.iter().join(" ");

    println!("{}", res );

}