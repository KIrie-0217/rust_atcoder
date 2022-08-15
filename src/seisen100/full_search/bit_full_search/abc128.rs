use proconio::input;
use std::collections::HashSet;

fn main(){

    input!{
        n:usize,
        m:usize,
    }
    let mut switch_table = Vec::new();

    for _ in 0..m {

        input!{
            k:usize,
            s:[ usize ; k]
        }
        let mut s_hash: HashSet<usize> = HashSet::new();
        for i in s.iter(){
            s_hash.insert(*i -1 );
        }
        switch_table.push(s_hash);

    }
    input!{ p:[ usize ; m]}

    let mut ans = 0;
    for bit in 0..( 1 << n){
        // let mut light :[ bool ; m] = ;
        let mut light = 0;
        for ( i , switches ) in switch_table.iter().enumerate(){
            // println!("{:?} {:?}",switches , switches.contains( &1 ));
            let on_switch_list = ( 0 .. n )
                .filter( |x| ( bit & ( 1 << x ) ) != 0 );
            let mut count:usize = 0 ;
            for j in on_switch_list{
                // print!("{} ", j);
                if switches.contains( &j ) {
                    count += 1
                }
            }
            // println!();
            if count%2 == p[i]{
                light += 1;
            }


        }
        // println!( "the light is {}", light );
        if light == m{
            ans += 1;
        }
    }
    println!("{}", ans);

}