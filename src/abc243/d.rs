use proconio::{input, marker::Chars};

fn main() {
    input! {
        _:usize,x:u128,
        s:Chars
    }

    let mut depth: usize = 0;
    let mut col: u128 = 0;
    loop {
        if x / 2_u128.pow(depth as u32) == 0 {
            depth -= 1;
            col = x % 2_u128.pow(depth as u32);
            break;
        }
        depth += 1;
    }
    let mut comp = vec![];
    for &c in s.iter() {
        match c {
            'U' => {
                if let Some(p) = comp.last().cloned() {
                    if p != 'U' {
                        comp.pop();
                        continue;
                    }
                }
                comp.push('U');
            }
            _ => {
                comp.push(c);
            }
        }
    }

    for c in comp {
        match c {
            'U' => {
                depth -= 1;
                col /= 2;
            }
            'L' => {
                depth += 1;
                col = 2 * col;
            }
            'R' => {
                depth += 1;
                col = 2 * col + 1;
            }
            _ => {
                continue;
            }
        }
    }

    let ans = 2_u128.pow(depth as u32) + col;
    println!("{}", ans);
}
