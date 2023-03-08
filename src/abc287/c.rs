use proconio::input;

fn dfs(now: usize, map_vec: &Vec<Vec<usize>>, arrive: &mut Vec<bool>) -> bool {
    arrive[now] = true;

    if arrive.iter().all(|x| *x == true) {
        return true;
    }

    if map_vec[now].len() > 2 {
        return false;
    } else if map_vec[now].len() == 2 {
        let point1 = map_vec[now][0];
        let point2 = map_vec[now][1];

        if !arrive[point1] {
            return dfs(point1, map_vec, arrive);
        } else if !arrive[point2] {
            return dfs(point2, map_vec, arrive);
        } else {
            return false;
        }
    } else {
        let point = map_vec[now][0];
        if !arrive[point] {
            return dfs(point, map_vec, arrive);
        } else {
            return false;
        }
    }
}

fn main() {
    input! {
        n:usize,m:usize
    }

    let mut map_vec: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        input! {
            v1:usize,
            v2:usize
        }
        map_vec[v1 - 1_usize].push(v2 - 1_usize);
        map_vec[v2 - 1_usize].push(v1 - 1_usize);
    }

    let mut start_index: usize = 0;
    let mut detect: bool = false;
    for index in 0..n {
        if map_vec[index].len() == 1 {
            start_index = index;
            detect = true;
            break;
        }
    }
    if !detect {
        println!("No");
        return;
    }

    let mut arrive: Vec<bool> = vec![false; n];
    if dfs(start_index, &map_vec, &mut arrive) {
        println!("Yes");
    } else {
        println!("No");
    };
    return;
}
