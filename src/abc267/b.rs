use proconio::input;

fn main(){

    input!{
        s:String
    }

    let mut pin_col = vec![0,0,0,0,0,0,0];
    let mut pin1_is_stand = true;

    // println!("{:?}", pin_col);
    for (i,pin) in s.as_str().chars().enumerate(){
        let pin_num = pin as i64 -48;
        // println!("{:?},{:?}",i , pin_num);
        if i==0 && pin == '0'{
            pin1_is_stand = false;
        }

        if i==0 || i==4{
            pin_col[3] += pin_num;
        }
        else if i==1 || i==7{
            pin_col[2] += pin_num;
        }
        else if i==2 || i==8{
            pin_col[4] += pin_num;
        }
        else if i==3{
            pin_col[1] += pin_num;
        }
        else if i==6 {
            pin_col[0] += pin_num;
        }
        else if i==5{
            pin_col[5] += pin_num;
        }
        else if i==9{
            pin_col[6] += pin_num;
        }
    }
    // println!("{:?}", pin_col);
    let mut isSplit = false;
    let mut count_start = false;
    let mut count:i64 = 0;
    for (i , col) in pin_col.iter().enumerate() { 

        if !count_start && col != &0{
            count_start  = true;
        }
        else if count_start && col == &0{
            count += 1;
        }else if count_start && col != &0{
            if count >= 1{
                isSplit = true;
            }
            count = 0;
        }
    }

    if isSplit && !pin1_is_stand{
        println!("Yes");
    }else{
        println!("No");
    }
}