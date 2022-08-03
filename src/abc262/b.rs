use proconio::input;

fn main() {
    input!{
        n:usize,
        m:usize,
        uv:[(usize, usize); m]
    }
    let mut tree_map = vec![ vec![false ; n]; n];
    for i in uv.iter().take(m){
        let (u ,v) = i;
        tree_map[ u - 1 ][ v- 1 ] = true;
        tree_map[ v - 1 ][ u -1 ] = true;
    }

    let mut count = 0;
    for i in 0..n{
        for j in (i + 1)..n{
            for k in (j + 1)..n{
                if tree_map[i][j] && tree_map[j][k] && tree_map[k][i]{
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

}
