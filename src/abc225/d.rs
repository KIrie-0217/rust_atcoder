use proconio::input;

fn main() {
    input! {
        n:usize,q:usize
    }

    let mut front: Vec<usize> = vec![0; n + 1];
    let mut back: Vec<usize> = vec![0; n + 1];

    let mut tmp: Vec<usize> = vec![];

    for _ in 0..q {
        input! {
            i:usize
        }
        match i {
            1 => {
                input! {
                    x:usize,y:usize,
                }
                front[y] = x;
                back[x] = y;
            }
            2 => {
                input! {
                    x:usize,y:usize,
                }
                front[y] = 0;
                back[x] = 0;
            }
            3 => {
                input! {
                    mut x:usize
                }
                tmp.clear();
                tmp.push(0);

                loop {
                    if front[x] == 0 {
                        break;
                    }
                    x = front[x];
                }

                loop {
                    tmp.push(x);
                    if back[x] == 0 {
                        break;
                    }
                    x = back[x];
                }
                tmp[0] = tmp.len() - 1;

                println!(
                    "{}",
                    tmp.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            _ => unreachable!(),
        }
    }
}
