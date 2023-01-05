use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        board : [ String; 9 ]
    }


    let mut vec_board = vec![ vec![0;9] ; 9];
    let mut porn_pos = Vec::new();
    for (i,row) in board.iter().enumerate(){
        for (j , point) in row.as_str().chars().enumerate(){
            
            if point == '#'{
                vec_board[i][j] += 1;
                porn_pos.push( vec![ i as i64 , j  as i64]);
            }
        }
    }


    let porns = porn_pos.len();
    let mut ans = 0 as usize;

    for comb in (0..porns).combinations(4){
        
        let a = porn_pos[ comb[0] ].clone();
        let b = porn_pos[ comb[1] ].clone();
        let c = porn_pos[ comb[2] ].clone();
        let d = porn_pos[ comb[3] ].clone();


        let ab_pow = ( a[0] - b[0]).pow(2) + ( a[1] -b[1]).pow(2); 
        let ac_pow = ( a[0] - c[0]).pow(2) + ( a[1] -c[1]).pow(2);
        let ad_pow = ( a[0] - d[0]).pow(2) + ( a[1] -d[1]).pow(2);

        let ba_pow = ab_pow.clone();
        let bc_pow = ( b[0] - c[0]).pow(2) + ( b[1] -c[1]).pow(2);
        let bd_pow = ( b[0] - d[0]).pow(2) + ( b[1] -d[1]).pow(2);

        let ca_pow = ac_pow.clone();
        let cb_pow = bc_pow.clone();
        let cd_pow =  ( c[0] - d[0]).pow(2) + ( c[1] -d[1]).pow(2);

        let mut a_length = vec![ ab_pow , ac_pow , ad_pow];
        let mut b_length = vec![ ba_pow , bc_pow ,bd_pow];
        let mut c_length = vec![ ca_pow , cb_pow , cd_pow];
        let mut d_length = vec![ ad_pow , bd_pow ,cd_pow];


        if ab_pow == ac_pow || ab_pow == ad_pow || ac_pow == ad_pow{
            
            let mut min_length = ab_pow.clone();
            let mut max_length = ab_pow.clone();

            min_length = min_length.min( ac_pow);
            min_length = min_length.min( ad_pow);

            max_length = max_length.max( ac_pow );
            max_length = max_length.max( ad_pow );

            let mut min_b = ba_pow.clone();
            let mut max_b = ba_pow.clone();

            min_b = min_b.min( bc_pow);
            min_b = min_b.min( bd_pow);

            max_b = max_b.max( bc_pow );
            max_b = max_b.max( bd_pow );

            let mut min_c = ca_pow.clone();
            let mut max_c = ca_pow.clone();

            min_c = min_c.min( cb_pow);
            min_c = min_c.min( cd_pow);

            max_c = max_c.max( cb_pow );
            max_c = max_c.max( cd_pow );

            let mut min_d = ad_pow.clone();
            let mut max_d = ad_pow.clone();

            min_d = min_d.min( bd_pow);
            min_d = min_d.min( cd_pow);

            max_d = max_d.max( bd_pow );
            max_d = max_d.max( cd_pow );



            if min_length * 2 == max_length{

                if max_b == max_length && max_c == max_length && max_d == max_length{
                    
                    if min_b == min_length && min_c == min_length && min_d == min_length{
                        ans += 1;
                    }
                }
                    
            }


        }

    }

    println!( "{}", ans);
}