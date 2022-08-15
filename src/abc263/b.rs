use proconio::input;

fn main(){


    input!{
        n:i32,
        p:[i32;n-1]
    }
    let mut parents_tree = Vec::new();
    parents_tree.push(-1);
    for i in &p{
        parents_tree.push( i -1 );
    }
    // println!("{:?}", parents_tree);

    let mut count = 0;
    let mut index = n-1;
    while index != -1 {
        index = parents_tree[ index as usize] ;
        count += 1;
       
    }
    println!("{}", count -1 );
}