use proconio::input;

fn cambus_check(cambus: &Vec<Vec<bool>>, i: usize, j: usize, h: usize, w: usize) -> bool {
    let mut check_point: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
    if i != 0 {
        check_point.push(vec![i - 1, j]);
    }

    if j != 0 {
        check_point.push(vec![i, j - 1]);
    }

    if i != h {
        check_point.push(vec![i + 1, j]);
    }

    if j != w {
        check_point.push(vec![i, j + 1]);
    }

    for point in check_point {
        if cambus[point[0]][point[1]] {
            return true;
        }
    }

    false
}

fn main() {
    input! {
        h:usize, w:usize,
        cambus:[String;h]
    }

    let mut cambus_bool: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    for i in 0..h {
        cambus_bool.push(vec![]);
        for tip in cambus[i].as_str().chars() {
            if tip == '.' {
                cambus_bool[i].push(false);
            } else {
                cambus_bool[i].push(true);
            }
        }
    }

    let mut check = true;
    for i in 0..h {
        if check {
            for j in 0..w {
                if cambus_bool[i][j] {
                    if !cambus_check(&cambus_bool, i, j, h - 1, w - 1) {
                        check = false;
                        break;
                    };
                }
            }
        }
    }

    if check {
        println!("Yes");
    } else {
        println!("No");
    }
}
