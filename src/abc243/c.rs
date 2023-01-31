use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n:u64,
        xy:[[u64;2];n],
        s:String
    }

    let mut y_r_collect_map: HashMap<u64, u64> = HashMap::<u64, u64>::new();
    let mut y_l_collect_map: HashMap<u64, u64> = HashMap::<u64, u64>::new();
    let direct_vec: Vec<usize> = s
        .chars()
        .map(|c| match c == 'R' {
            true => 0,
            false => 1,
        })
        .collect();
    for i in 0..n as usize {
        if direct_vec[i] == 0 {
            y_r_collect_map
                .entry(xy[i][1])
                .and_modify(|x_min| {
                    if *x_min > xy[i][0] {
                        *x_min = xy[i][0];
                    }
                })
                .or_insert(xy[i][0]);
        } else {
            y_l_collect_map
                .entry(xy[i][1])
                .and_modify(|x_max| {
                    if *x_max < xy[i][0] {
                        *x_max = xy[i][0];
                    }
                })
                .or_insert(xy[i][0]);
        }
    }

    let mut is_collision = false;
    for key in y_r_collect_map.keys() {
        if y_l_collect_map.contains_key(key) {
            if y_r_collect_map[key] < y_l_collect_map[key] {
                is_collision = true;

                break;
            }
        }
    }

    if is_collision {
        println!("Yes");
    } else {
        println!("No");
    }
}
