use proconio::input;

fn main(){

    input!{
        s:String,
        t:String 
    }
    
    let  s_chars : Vec<char>= s.chars().collect();
    let  t_chars : Vec<char>= t.chars().collect();
    for ( i , c ) in s_chars.iter().enumerate(){
        if c != &t_chars[i] {
            println!("{}", i+1);
            return
        }
        
    }
    println!("{}", t_chars.len() );
    return
}