use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
        s_vec:[ String; n]
    }

    let mut ans: Vec<String> = Vec::<String>::new();
    for i in 0..k {
        ans.push(s_vec[i].clone());
    }
    ans.sort_by(|a, b| a.cmp(b));

    for s in ans {
        println!("{}", s);
    }
}
