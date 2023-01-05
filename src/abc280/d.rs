use proconio::input;

fn calc_divisors( k: i128 ) -> Vec<i128>{
    let mut res : Vec<i128> = Vec::new();

    for i in 1..k+1 {
        if i*i > k { break }

        if k % i != 0 {
            continue;
        }else{
            res.push( i );

            if  k/i != i{
                res.push( k/i );
            }
        }

        res.sort()
    }

    return res
}

fn is_prime( target : i128) -> bool{
    if target < 2{
        return false
    }else{
        let mut i = 2;
        while i*i < target{
            if target % i == 0 {
                return false
            }
            i += 1 as i128;
        }

        return true
    }
}

fn main(){

    input!{
        k:i128
    }

    let mut tmp:i128 = 1;
    for i in  calc_divisors(k){
        tmp *= i;
        if tmp == k{
            println!("{}", i);
            return;
        }
    }

    println!("{}", )
}