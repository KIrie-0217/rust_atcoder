use proconio::input;

fn row_to_col( s:Vec<String> ) -> Vec<Vec<char>>{
    let mut res : Vec<Vec<char>> = vec![ vec![ '.'; s.len() ] ; s[0].len() ] ;
    for ( i , s_char )in s.iter().enumerate(){
        for (j , c) in s_char.chars().enumerate(){
            res[j][i] = c;
        }
    }
    
    res
}

fn main(){

    input!{
        h:usize,
        w:usize,
        s: [ String; h],
        t: [ String; h]
    }

    let mut col_s = row_to_col( s );
    let mut col_t = row_to_col( t );

    col_s.sort();
    col_t.sort();

    let  checker : bool = if col_s == col_t {true} else{ false };
    
    if checker{
        println!("Yes");
    }else{
        println!("No");
    }
    

}