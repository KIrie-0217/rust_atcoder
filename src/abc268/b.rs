use proconio::input;

fn main(){

    input!{
        s:String,
        t:String
    }

    

   
    let mut check = if s.len()>t.len() { false }else{true};
    let mut t = t.chars();
    
    if check{
        for c in s.chars(){
            let t_get = &t.nth(0).unwrap();
            if t_get != &c{
                check = false;
            }
        }
    }    
    if check{
        println!("Yes");
    }else{
        println!("No");
    }
}