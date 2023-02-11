use proconio::input;

fn red_jewel(level: usize, red: usize, blue: usize) -> usize {
    if level == 1 {
        return 0 as usize;
    } else {
        let ans = red_jewel(level - 1, red, blue) + blue_jewel(level, red, blue) * red;
        return ans;
    }
}

fn blue_jewel(level: usize, red: usize, blue: usize) -> usize {
    if level == 1 {
        return 1 as usize;
    } else {
        let ans = red_jewel(level - 1, red, blue) + blue_jewel(level - 1, red, blue) * blue;
        return ans;
    }
}

fn main() {
    input! {
        n:usize,x:usize,y:usize
    }

    let blue_count = red_jewel(n, x, y);

    println!("{}", blue_count);
}
