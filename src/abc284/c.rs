use proconio::input;

fn main() {
    input! {
        n:usize,m:usize,
    }
    if m == 0 {
        println!("{}", n);
        return;
    } else {
        input! {
         v_arr:[ [usize;2];m]
        }

        let mut v_map: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();
        let mut check_vec: Vec<bool> = Vec::<bool>::new();
        for _ in 0..n {
            v_map.push(vec![]);
            check_vec.push(false);
        }

        for point in v_arr {
            v_map[point[0] - 1].push(point[1].clone() - 1);
            v_map[point[1] - 1].push(point[0].clone() - 1);
        }
        fn dfs(v_map: &Vec<Vec<usize>>, next: usize, check_map: &mut Vec<bool>) {
            check_map[next] = true;

            for i in v_map[next].iter() {
                if !check_map[i.clone()] {
                    dfs(&v_map, i.clone(), check_map);
                }
            }
        }

        let mut count: usize = 0;
        for i in 0..n {
            if !check_vec[i] {
                dfs(&v_map, i, &mut check_vec);
                count += 1
            }
        }

        println!("{}", count);
    }
}
