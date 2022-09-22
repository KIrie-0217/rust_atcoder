use proconio::input;
use itertools::Itertools;

fn main(){

    input!{
        a:(i64,i64),
        b:(i64,i64),
        c:(i64,i64),
        d:(i64,i64)
    }

    let abc: i64 = (a.1 - b.1)*  (c.0 - b.0)  - (a.0 - b.0)*(c.1 - b.1);
    let bcd: i64 = (b.1 - c.1) * (d.0 - c.0) - (b.0 - c.0 )*(d.1 - c.1);
    let cda: i64 = (c.1 - d.1) * (a.0 - d.0) - (c.0 - d.0) * (a.1 - d.1);
    let dab: i64 = (d.1 - a.1) * (b.0 - a.0 ) - (d.0 - a.0)*( b.1 - a.1);

    if abc > 0 && bcd >0 && cda > 0 && dab > 0 {
        println!("Yes");
    }else{
        println!("No");
    }
}