use proconio::input;

fn main(){

    input!{
        n:usize,
        m:usize,
        xy:[ (i32 , i32) ; m ]
    }

    let mut friendship = vec![ 0usize; n];
    for i in 0..n{
        friendship[i] = 1 << i; // 自分の位置を1にする
    }
    // println!("{:?}", friendship);

    for ( x, y ) in xy {
        friendship[ x as usize -1 ] += 1 << (y-1);
        friendship[ y as usize -1 ] += 1 << (x-1);
    } // 知り合いの位置を１にする

    // bit全探索をして全部のfriendshipが同じならTrue・最大値を返す
    let mut ans:i32 = 0;
    for i in 0..( 1<<(n) ){
        let mut tmp = i.clone();
        let mut count = 0;
        let friend_list = ( 0 .. n )
        .filter( |x| ( i as i32  & ( 1 << x ) ) != 0 );

        for j in friend_list{
            count += 1;
            tmp &= friendship[j];
        }

        if tmp == i{
            ans = ans.max( count );
        }
    }

    println!("{:?}", ans );


}