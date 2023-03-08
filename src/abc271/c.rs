use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[u64;n]
    }

    let mut keep: Vec<bool> = vec![false; n + 2];
    let mut sell: usize = 0;
    for i in 0..n {
        if a[i] >= keep.len() as u64 {
            sell += 1;
        } else if keep[a[i] as usize] {
            sell += 1;
        } else {
            keep[a[i] as usize] = true;
        }
    }

    let mut min_not_have: usize = 1;
    let mut max_have: usize = n + 1;
    loop {
        while keep[min_not_have] {
            min_not_have += 1;
        }
        while max_have != 0 && !keep[max_have] {
            max_have -= 1;
        }

        if sell >= 2 {
            sell -= 2;
            keep[min_not_have] = true;
        } else {
            if min_not_have >= max_have {
                break;
            }
            keep[max_have] = false;
            sell += 1;
        }
    }

    println!("{}", min_not_have - 1);
    return;
}
