use proconio::input;

fn main(){

    input!{
        s:String
    }

    let mut ans = 0;
    let mut tmp = 0;

    for i in s.as_str().chars(){
        if "ACGT".contains( i ){
            tmp += 1;
            ans = ans.max( tmp );
        }else{
            tmp = 0;
        }
    }

    println!("{}",ans);

}