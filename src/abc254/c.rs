use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
        mut a:[usize;n]
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut k_split_vec: Vec<Vec<usize>> = vec![Vec::<usize>::new(); k];
    for i in 0..n {
        k_split_vec[i % k].push(a[i])
    }

    for i in 0..k {
        k_split_vec[i].sort_by(|a, b| b.cmp(&a));
    }

    let mut reverse_a: Vec<usize> = vec![];
    for i in 0..n {
        reverse_a.push(k_split_vec[i % k].pop().unwrap());
    }

    a.sort_by(|a, b| a.cmp(&b));
    let mut ans = true;
    for i in 0..n {
        if a[i] != reverse_a[i] {
            ans = false;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }

    return;
}
