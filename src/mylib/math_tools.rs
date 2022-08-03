use cargo_snippet::snippet;

#[snippet(prefix = "use math_tools::*;") ]
pub mod math_tools{
    use num::Integer;

    pub fn divisor( n: usize )  -> Vec<usize>{
        assert!( n >= 1 );
        let mut retval: Vec<usize> = Vec::new();
        for i in 1..= num_integer::sqrt( n ){
            if n.is_multiple_of(&i){
                retval.push( i );
                if i*i != n{
                    retval.push( n /i );
                }
            }
        }
        retval.sort();
        retval
    }
}