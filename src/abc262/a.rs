use proconio::input;

fn main() {
    input!{
        y:usize,
    }

    for i in y..(y+4){
        if i%4 == 2{
            print!("{}",i);
            break;
        }
    }
}
