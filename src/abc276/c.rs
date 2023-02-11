use std::any::type_name;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        p:[usize;n]
    }

    let mut p: Vec<usize> = Vec::<usize>::from(p);

    let mut tmp: usize = n - 2;
    while p[tmp] < p[tmp + 1] {
        tmp -= 1;
    }

    let mut tmp_index: usize = n - 1;
    let mut max_index: usize = 0;

    let mut tmp_max: usize = 0;
    while tmp < tmp_index {
        if tmp_max < p[tmp_index] && p[tmp_index] < p[tmp] {
            max_index = tmp_index;
            tmp_max = p[tmp_index];
        }

        tmp_index -= 1;
    }

    let before_num = p[tmp];
    let after_num = p[max_index];
    p[tmp] = after_num;
    p[max_index] = before_num;

    println!(
        "{} {}",
        &p[0..=tmp].iter().join(" "),
        &p[tmp + 1..n].iter().rev().join(" ")
    );
}
