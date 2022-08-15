use proconio::input;

fn main(){

    input!{

        cards: [ i32 ; 5]
    }

    let mut card_append = vec![ 0 ; 13];
    for i in cards{
        
        card_append[ (i -1) as usize] += 1;
        
    }
    if card_append.contains(&2) && card_append.contains(&3){
        println!("Yes");
    }else{
        println!("No");
    }


}