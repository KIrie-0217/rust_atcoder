use proconio::input;

fn recurcive_create(start_num: usize, base_num: usize, create_num: usize) -> usize {
    let total: usize = start_num;
    let hold: usize = total % base_num;
    let new_num: usize = total / base_num * create_num;
    if base_num > new_num + hold {
        return new_num;
    }

    let tmp = recurcive_create(new_num + hold, base_num, create_num);
    return new_num + tmp;
}

fn main() {
    input! {
        m:usize,
        n:usize,
        first_num:usize
    }
    let ans = recurcive_create(first_num, m, n) + first_num;

    println!("{}", ans);
}
