use proconio::input;

fn main(){
    input!{

        n:usize, m:usize,
        a:[ [ usize ; m] ; n ]

    }

    let mut ans = 0;
    for i in 0..m{
        for j in i+1..m{
            let mut tmp = 0;
            for k in 0..n{
                tmp += a[k][i].max( a[k][j] );
            }
            ans = ans.max( tmp );
        }
    }
    println!("{}", ans);
}