use proconio::input;
use proconio::marker::Chars;

fn main(){

    input!{
        h:usize, w:usize,
        g:[ Chars ; h]
    }

    let mut already_came = Vec::new();
    for _i in 0..h{
        let mut tmp = Vec::new();
        for _j in 0..w{
            tmp.push(false);
        }
        already_came.push(tmp);
    }
    // println!("{:?}",already_came);
    let mut coordinate = ( 0usize , 0usize );

    loop{

        let where_to_go =  g[coordinate.0][coordinate.1];
        match where_to_go {

            'U' => {
                if coordinate.0 == 0usize{
                    println!("{} {}", coordinate.0 + 1usize, coordinate.1 + 1usize );
                    break
                }else{
                    coordinate.0 -= 1usize;
                }
            }
            'D' => {
                if coordinate.0 == h -1{
                    println!("{} {}", coordinate.0 + 1usize, coordinate.1 + 1usize );
                    break
                }else{
                    coordinate.0 += 1usize;
                }
            }
            'R' => {
                if coordinate.1 == w -1{
                    println!("{} {}", coordinate.0 + 1usize, coordinate.1 + 1usize );
                    break
                }else{
                    coordinate.1 += 1usize;
                }
            }
            'L' => {
                if coordinate.1 == 0{
                    println!("{} {}", coordinate.0 + 1usize, coordinate.1 + 1usize );
                    break
                }else{
                    coordinate.1 -= 1usize;
                }
            }
            _ => break
        }

        if already_came[coordinate.0][coordinate.1] == true{
            println!("{}", -1);
            break
        }else{
            already_came[coordinate.0][coordinate.1] = true
        }

    }
    
}