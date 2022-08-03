use std::collections::HashSet;
use std::time::{Duration, Instant};
use rand;
use rand::prelude::*;

macro_rules! measure {
    ( $x:expr) => {
       {
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(" {:011}",  end.subsec_nanos()  );
        result
       } 
    };
}

fn generate( x:i32){
    let mut vec_x = Vec::new();
    let mut set_x = HashSet::new();

    measure!(    {
        for i in 0..x{
            set_x.insert( (i , i) );
        }
        print!("set {} genetation:",x)
    }     );

    measure!(    {
        for i in 0..x{
            vec_x.push( (i , i) );
        }
        print!("vec {} genetation:",x)
    }     );
}

fn contains(x:i32){
    let mut vec_x = Vec::new();
    let mut set_x = HashSet::new();
    for i in 0..x{
        set_x.insert( (i , i) );
        vec_x.push( (i , i) );
    }

    let seed :[ u8; 32 ]= [13;32];
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(seed);
    for i in 0..10{
        let randm_num = rng.gen::< f32 >();
        let num = randm_num * x as f32;
        measure!( {
            set_x.contains( &(num as i32, num as i32) );
            print!(" set {}_{} contains:", x,i)
        });
        measure!( {
            vec_x.contains( &(num as i32, num as i32) );
            print!(" vec {}_{} contains:", x,i)
        });
    }
}

fn assignment(x:i32){
    let mut vec_x = Vec::new();
    let mut set_x = HashSet::new();
    for i in 0..x{
        set_x.insert( (i , i) );
        vec_x.push( (i , i) );
    }

    measure!(    {
        for mut i in set_x.iter(){
             i  = &( 1 ,1 );
        }
        print!("set {} assignment:",x)
    }     );

    measure!(    {
        for mut i in vec_x.iter(){
             i  = &( 1 ,1 );
        }
        print!("vec {} assignment:",x)
    }     );
}

fn iter_check(x:i32){
    let mut vec_x = Vec::new();
    let mut set_x = HashSet::new();
    for i in 0..x{
        set_x.insert( (i , i) );
        vec_x.push( (i , i) );
    }

    measure!(    {
        for mut i in set_x.iter(){}
        print!("set {} iter:",x)
    }     );

    measure!(    {
        for mut i in vec_x.iter(){}
        print!("vec {} iter:",x)
    }     );
}

fn main(){
    
    let length = [ 10,100,1000,10_000,100_000,100_000];
    for i in length.iter(){
        generate(*i);

        contains( *i );

        assignment(*i);

        iter_check(*i);
    }

}
