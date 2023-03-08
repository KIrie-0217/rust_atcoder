use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        q:usize
    }

    let mut cylinder: VecDeque<Vec<u128>> = VecDeque::<Vec<u128>>::new();
    for _ in 0..q {
        input! {
            q_type:usize
        }
        match q_type {
            1_usize => {
                input! {
                    x:u128,c:u128
                }
                cylinder.push_back(vec![c, x]);
            }
            2_usize => {
                input! {
                    mut c:u128
                }
                let mut q_ans: u128 = 0;
                loop {
                    if let Some(pick_up) = cylinder.pop_front() {
                        if pick_up[0] < c {
                            c -= pick_up[0];
                            q_ans += pick_up[1] * pick_up[0];
                        } else {
                            let remain: u128 = pick_up[0] - c;
                            q_ans += pick_up[1] * c;

                            if remain != 0 {
                                cylinder.push_front(vec![remain, pick_up[1]]);
                            }
                            break;
                        }
                    };
                }

                println!("{}", q_ans);
            }
            _ => unreachable!(),
        }
    }
}
