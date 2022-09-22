use proconio::input;

fn main(){

    input!{
        n:i64, k:i64,
        a: [ i64; n]
    }



    let mut ans = std::i64::MAX;
    for i in 0..( 1<<n ) {
        let mut cost = 0;
        let mut watching = 1;
        let mut height = a[0];
        for j in 1..n{

            if i >> j &1 == 1{ // iを右にjシフトさせる（≒j番目の値）が１かチェックする
                if a[j as usize ] > height{
                    height = a[j as usize ];
                    watching += 1;
                }else{
                    cost += height+ 1 - a[j as usize ];
                    height += 1;
                    watching += 1;
                }

            }else if a[j as usize ] > height {
                height = a[j as usize ];
            }

        }
        if watching >= k {
            ans = ans.min( cost );
        }
    }
    println!("{}", ans);

}