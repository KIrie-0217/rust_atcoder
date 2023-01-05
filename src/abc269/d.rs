use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;

fn adjacent_point( current: &Vec<i32> ) -> Vec< Vec<i32> >{
    let mut out :Vec< Vec<i32> >= Vec::new();

    out.push( vec![ current[0]-1,current[1]-1]);
    out.push( vec![ current[0]-1,current[1]]);
    out.push( vec![ current[0],current[1]-1]);
    out.push( vec![ current[0],current[1]+1]);
    out.push( vec![ current[0]+1,current[1]]);
    out.push( vec![ current[0]+1,current[1]+1]);
    
    out
}

fn dfs(
    map:&HashMap<usize, Vec<usize>>,
    current:usize,
    seen: &mut Vec<bool>
){
    seen[current] = true;
    for node in &map[&current]{
        if node != &current && !seen[*node] {
            dfs(&map, *node ,seen);
        }
    }
}

fn main(){

    input!{
        n:usize,
        xy: [ [i32;2];n]
    }
    let mut black_points_index: HashMap< &Vec<i32>, usize> = HashMap::new();
    for ( i, bp ) in xy.iter().enumerate(){
        black_points_index.entry(bp).or_insert(i);
    }


    let mut black_points_map:HashMap<usize, Vec<usize> > = HashMap::new();
    for (i , bp) in xy.iter().enumerate(){
        black_points_map.insert(i, vec![i]);        

        let adj = adjacent_point( &bp );
        for j in adj.iter(){
            if black_points_index.contains_key(j){
                black_points_map.entry( i ).or_insert(vec![]).push( black_points_index[j]);
            }
        }
    }

    let mut seen:Vec<bool> = vec![false;n];
    let mut count: usize = 0;
    for i in 0..n{
        if seen[i] == false{
            dfs( &black_points_map , i, &mut seen);
            count += 1;
        }
    }

    println!("{}", count);

 
}