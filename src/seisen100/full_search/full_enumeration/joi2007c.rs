use proconio::input;
use std::collections::HashSet;

fn main(){

    input!{
        pillar:i32,
        xy: [ (i32, i32) ; pillar]
    }
    let mut set = HashSet::new(); // vec.contains はなんかめちゃくちゃ遅い
    for i in &xy {
        set.insert( i );
    }

    // println!("{:?}", xy);
    // let a:Vec<i64> = vec![9,2];
    // println!("{}", xy.contains( &a )  );

    let mut ans = 0;
    let mut area;
    for i in 0..pillar-1{
        let (p1_x ,p1_y )  = &xy[i as usize] ;
        for j in i+1..pillar{
            let (p2_x ,p2_y )  = &xy[j as usize] ;

            area = (p2_x-p1_x).pow(2) +(p2_y - p1_y).pow(2) ;
            if area < ans{
                continue;
            }


            let ( x_distance , y_distance ) :( i32 , i32)  = ( (p2_x - p1_x) , (p2_y - p1_y) );

            let upper_p1 = ( p1_x - y_distance , p1_y + x_distance );
            let upper_p2 = ( p2_x - y_distance , p2_y + x_distance );

            if set.contains(&upper_p1) && set.contains(&upper_p2){
                ans = ans.max( area);
                // break;
            }

            // let lower_p1:Vec<i64> = vec![ p1[0] + y_distance , p1[1] - x_distance ];
            // let lower_p2:Vec<i64> = vec![ p2[0] + y_distance , p2[1] - x_distance ];

            // if xy.contains(&lower_p1) && xy.contains(&lower_p2){
            //     area =  x_distance.pow(2) + y_distance.pow(2) ;
            //     ans = ans.max( area );
            //     break;
            // }

        }
    }
    println!("{}", ans as usize);
}