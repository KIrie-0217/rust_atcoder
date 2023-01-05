use proconio::input;

fn main(){

    input!{
        s:String
    }
    
    let s_vec  : Vec<char> = s.chars().collect();

    if s_vec.len() != 8 {
        println!("No");
        return ;
    }

    for (i  , c )in s_vec.iter().enumerate(){
        if i == 0{
            if c.is_ascii_uppercase(){
                continue;
            }else{
                println!("No");
                return ;
            }
        }

        else if i == 1{
            if c.is_ascii_digit(){

                if c == &'0'{
                    println!("No");
                    return;
                }
                else{
                    continue;
                }
            }else{
                println!("No");
            }
        }

        else if i > 1 && i <= 6{
            if c.is_ascii_digit(){
                continue;
            }else{
                println!("No");
                return;
            }
        }

        else{
            if c.is_ascii_uppercase(){
                continue;
            }else{
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
    return;
}