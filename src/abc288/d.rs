use proconio::input;

fn get_data(a_part: &Vec<i128>, raq: &Vec<i128>, index: usize) -> i128 {
    let mut ans: i128 = 0;
    for i in 0..=index {
        ans += raq[i];
    }
    ans += a_part[index];
    ans
}

fn main() {
    input! {
        n:usize,k:usize,
        a:[i128;n],
        q:usize,
    }

    let a_vec: Vec<i128> = Vec::<i128>::from(a);
    for _ in 0..q {
        input! {
            lr: [usize;2]
        }

        let mut a_part = a_vec[lr[0] - 1..lr[1]].to_vec();
        let size: usize = lr[1] - lr[0] + 1;
        let mut raq: Vec<i128> = vec![0; size + 1];
        a_part.push(0);

        // println!("{}", size);
        // println!("{:?}", a_part);
        for i in 0..size {
            let tmp = -1 * get_data(&a_part, &raq, size - i - 1);
            let start = size - i - k;
            let end = size - i;
            //   println!("{} {}", size - i - k, size - i);
            raq[start] += tmp;
            raq[end] -= tmp;

            if start == 0 {
                break;
            }
        }

        let mut ans: bool = true;
        for i in 0..size {
            if get_data(&a_part, &raq, i) != 0 {
                ans = false;
                break;
            }
        }

        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
