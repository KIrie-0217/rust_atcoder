use proconio::input;

fn main() {
    input! {
        n:usize,
        section:[[f64;3];n]
    }

    let mut sec_vec: Vec<Vec<f64>> = Vec::<Vec<f64>>::new();
    for i in 0..n {
        let pattern: usize = section[i][0].clone() as usize;
        let mut x: f64 = section[i][1].clone();
        let mut y: f64 = section[i][2].clone();

        match pattern {
            2 => y -= 0.5,
            3 => x += 0.5,
            4 => {
                x += 0.5;
                y -= 0.5;
            }
            _ => (),
        }

        let mut tmp: Vec<f64> = Vec::<f64>::new();
        tmp.push(x.clone());
        tmp.push(y.clone());
        sec_vec.push(tmp);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in i + 1..n {
            let section_1: &Vec<f64> = &sec_vec[i];
            let section_2: &Vec<f64> = &sec_vec[j];

            if section_1[0].max(section_2[0]) <= section_1[1].min(section_2[1]) {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
