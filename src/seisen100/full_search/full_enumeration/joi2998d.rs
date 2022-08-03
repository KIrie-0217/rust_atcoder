use proconio::input;
use std::collections::HashSet;

fn main(){
    input!{
        m:usize,
        star_xy : [ (i64,i64);m],
        n:usize,
        pic_xy : [ (i64,i64);n]
    }

    let mut set = HashSet::new();

    for &( px , py) in pic_xy.iter() {
        set.insert( (px, py) );
    }

    let standart_star = star_xy[0];

    for &( px , py ) in pic_xy.iter() {
        let mut is_continue = true;
        let ( dif_x , dif_y ) = ( px - standart_star.0  , py - standart_star.1 );

        for &( other_x , other_y ) in star_xy.iter(){
            let point:( i64, i64 ) = ( other_x + dif_x , other_y + dif_y );

            if !set.contains(&point){
                // println!("{:?}", point);
                // println!("{:?}", set);
                // println!("{}" , point == pic_xy[0]);
                is_continue = false;
                break;
            }
        }

        if !is_continue{
            //println!("continue");
            continue;
        }

        println!( "{} {}",dif_x,dif_y);
        break;
    }
}