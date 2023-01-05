use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        n:usize,m:usize,
        can_solve: [ String;n]
    }
   
    let mut solves :Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    for solve in  can_solve{
        let mut tmp : Vec<bool> = Vec::<bool>::new();
        for peace in solve.as_str().chars(){
            if peace == 'o'{
               tmp.push(true); 
            }else{
                tmp.push(false);
            }
        }
        solves.push(tmp);
    }

    
    let mut ans = 0;
    for pairs in (0..n).combinations(2){
        let mut check : bool = true;
        for i in 0..m{
            

            if !( solves[ pairs[0] ][i] | solves[ pairs[1]][i] ){
                check = false;
                break
            }
        }

        if check{
           ans += 1 ; 
        }
        
    }

    println!("{}",ans);
}