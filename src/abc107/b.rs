use proconio::input;

fn main() {
    input! {
        h:usize,w:usize,
        a:[String;h]
    }

    let mut map_vec: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    for i in 0..h {
        map_vec.push(vec![]);
        for c in a[i].chars() {
            map_vec[i].push(c.clone());
        }
    }

    let mut remove_row: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    let mut rows: usize = 0;
    for row in map_vec {
        if !row.iter().all(|x| *x == '.') {
            remove_row.push(row);
            rows += 1;
        }
    }

    // println!("{:?}", remove_row);

    let mut remove_col: Vec<usize> = Vec::<usize>::new();
    for i in 0..w {
        let mut is_col_all: bool = true;
        for j in 0..rows {
            if remove_row[j][i] == '#' {
                is_col_all = false;
                break;
            }
        }

        if is_col_all {
            remove_col.push(i);
        }
    }

    let mut ans: Vec<Vec<char>> = Vec::<Vec<char>>::new();
    for i in 0..rows {
        ans.push(vec![]);
        for j in 0..w {
            if !remove_col.contains(&j) {
                ans[i].push(remove_row[i][j]);
            }
        }
    }

    for row in ans {
        println!(
            "{}",
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
