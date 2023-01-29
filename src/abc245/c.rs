use proconio::input;

fn dfs(
    current_vec: &mut Vec<i64>,
    current_index: usize,
    last_index: usize,
    a_vec: &Vec<i64>,
    b_vec: &Vec<i64>,
    k: i64,
) -> bool {
    if current_index == last_index {
        return true;
    }
    let mut checker: bool = false;
    if (current_vec[current_index] - a_vec[current_index + 1]).abs() <= k {
        current_vec.push(a_vec[current_index + 1].clone());
        checker = dfs(
            current_vec,
            current_index + 1,
            last_index,
            &a_vec,
            &b_vec,
            k,
        );
    }

    if (current_vec[current_index] - b_vec[current_index + 1]).abs() <= k && !checker {
        current_vec.push(b_vec[current_index + 1].clone());
        checker = dfs(
            current_vec,
            current_index + 1,
            last_index,
            &a_vec,
            &b_vec,
            k,
        );
    }

    if !checker {
        current_vec.pop();
    }

    return checker;
}

fn main() {
    input! {
        n:usize,
        k:i64,
        mut a_vec :[i64;n],
        mut b_vec :[i64;n]
    }

    let mut n_vec: Vec<i64> = Vec::<i64>::new();
    let mut checker: bool;

    n_vec.push(a_vec[0].clone());
    checker = dfs(&mut n_vec, 0 as usize, n - 1, &a_vec, &b_vec, k);

    if !checker {
        n_vec.pop();
        n_vec.push(b_vec[0].clone());
        checker = dfs(&mut n_vec, 0 as usize, n - 1, &a_vec, &b_vec, k);
    }

    if checker {
        println!("Yes");
    } else {
        println!("No");
    }
}
