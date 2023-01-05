use proconio::input;
use std::collections::HashMap;

fn main(){

    input!{
        n:usize, x:usize, y:usize,
        vertax: [ [usize;2] ; n-1 ]
    }

    let mut tree:HashMap< usize, Vec<usize> > = HashMap::new();

    for point in vertax{
        tree.entry( point[0] )
            .or_insert( vec![  ] )
            .push( point[1] ) ;
        
        tree.entry( point[1] )
            .or_insert( vec![])
            .push( point[0] );
    }

    
    fn dfs(
        tree:&HashMap<usize, Vec<usize> >,
        current:usize,
        goal:usize,
        arr: &mut Vec<usize>,
        before:usize ){

            if current == goal{
                println!("{}", arr.iter()
                                    .map( |v| ( v ).to_string() )
                                    .collect::<Vec<String>>()
                                    .join(" ")
                        )
            }

            for &node in &tree[ &current ]{
                if node != before {
                    arr.push( node );
                    dfs( tree , node , goal , arr , current );
                    arr.pop();
                }
            }
    }

    dfs( &tree , x , y, &mut vec![x], 1_000_000_000);
}