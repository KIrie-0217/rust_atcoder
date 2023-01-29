use proconio::input;

fn main(){

    input!{
        _n:usize,
        s:String
    }

    let mut is_suround : bool = true;
    for  c in s.as_str().chars(){
        if c == '\"'{
           is_suround = !is_suround; 
        }
       
        if is_suround && c == ','{
            print!("{}",".");
        }else{
            print!("{}", c);
        }
        
    }
    
}