use proconio::input;

fn main() {
    input! {
        n:usize,m:usize
    }
    if m == 0 {
        let mut ans: Vec<usize> = vec![];
        for i in 1..=n {
            ans.push(i);
        }
        println!(
            "{}",
            ans.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    }

    input! {
        re : [usize;m]
    }
    let mut split: Vec<Vec<usize>> = vec![vec![]];

    for i in 1..n {
        if re.contains(&i) {
            if let Some(last) = split.last_mut() {
                last.push(i);
            }
        } else {
            if let Some(last) = split.last_mut() {
                last.push(i);
            }
            split.push(vec![]);
        }
    }
    if let Some(last) = split.last_mut() {
        last.push(n);
    }

    let mut ans: Vec<String> = vec![];
    for tip in split {
        ans.push(
            tip.iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
