use proconio::input;

fn main(){

    input!{
        s:String
    }
    let week = &s;


    if week == "Monday"{
        println!("5");
    }
    else if week == "Tuesday" {
        println!("4");
    }else if week == "Wednesday" {
        println!("3") ;
    }else if week == "Thursday" {
        println!("2") ;
    }else if week == "Friday" {
        println!("1");
    }
}