use proconio::input;

fn main(){

    input!{
        r:i64,
        c:i64
    }

    let raw = r -1 -7;
    let col = c -1 -7;

    let raw_abs = raw.abs();
    let col_abs = col.abs();
    if raw_abs == col_abs  {
        let _check = col_abs;
        if _check %2 == 0{
            println!("white");
        }
        else{
            println!("black");
        }
        return
    }
    let _check = raw_abs.max(col_abs);
    if _check%2 == 0 {
        println!("white");
    }else{
        println!("black");
    }

}