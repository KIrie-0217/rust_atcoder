use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q:usize,
    }

    let mut bag: BinaryHeap<Reverse<i128>> = BinaryHeap::<Reverse<i128>>::new();
    let mut offset: i128 = 0;
    for _ in 0..q {
        input! {
            i:usize
        }
        match i {
            1 => {
                input! {
                    xi:i128
                }
                bag.push(Reverse(xi - offset));
            }
            2 => {
                input! {
                    xi:i128
                }
                offset += xi;
            }
            3 => match bag.pop() {
                Some(n) => println!("{}", n.0 + offset),
                None => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
