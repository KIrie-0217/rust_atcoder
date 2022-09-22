use proconio::input;

fn flip_raw( senbei:&Vec<Vec<usize>> , index:usize) -> Vec< Vec<usize> > {
    let mut new_senbei = Vec::new();
    for (i , raw) in senbei.iter().enumerate(){
        let mut senbei_raw = Vec::new();
        if i == index{
            for j in raw{
                senbei_raw.push( j^1 );
            }
        }else{
            for j in raw{
                senbei_raw.push( *j );
            }
        }
        new_senbei.push( senbei_raw );
    }
    new_senbei
}

fn vec_sum( vector: &Vec<usize> ) -> usize {
    let mut count = 0;
    for i in vector{
        count += i;
    }
    count
}

fn col_sum( vector: &Vec< Vec<usize>>, index:usize) -> usize{
    let mut count = 0;
    for i in vector{
        count += i[index];
    }
    count
}


fn main(){

    input!{
        r:usize,
        c:usize,
        senbei:[ [ usize ;  c] ; r ]
    }

    let mut ans = 0;
    for i in 0..( 1 << r ) { // cに対してbit全探索しようとするとオーバーフローする
        // println!("----------");
        let mut new_senbei = senbei.clone();
        let mut count = 0;

        let flip_list = ( 0 .. r )
            .filter( |x| ( i as i32  & ( 1 << x ) ) != 0 );

        // 行方向を反転する
        for i in flip_list{
            new_senbei = flip_raw( &new_senbei , i);
        }

        // 列方向は全探索して最大値を取る
        for i in 0..c{
            let sum = col_sum( &new_senbei, i );
            let sum_flip = r - sum;
            // println!("{:?}",  i );
            // println!("{} {}", sum, sum_flip);
            count += sum.max(sum_flip);
        }

        ans = ans.max( count );
    }
    println!("{}",  ans );
}