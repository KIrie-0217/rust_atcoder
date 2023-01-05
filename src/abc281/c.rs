use proconio::input;

fn main(){

    input!{
        n:usize,
        t:usize,
        play_list: [ usize ;n]
    }

    let play_total_length : usize = play_list.iter().sum();


    let mut elasp_time:usize =   t % play_total_length;

    for (i , a ) in  play_list.iter().enumerate(){
        if elasp_time < *a{
            println!("{} {}", i+1 , elasp_time);
            return ;
        }else{
            elasp_time  -= a;
        }
    }

}