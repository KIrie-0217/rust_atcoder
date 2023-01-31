use proconio::input;

fn main() {
    input! {
        n:usize,
        txy:[[i64; 3];n]
    }

    let mut now_t: i64 = 0;
    let mut now_x: i64 = 0;
    let mut now_y: i64 = 0;

    let mut next_t: i64;
    let mut next_x: i64;
    let mut next_y: i64;

    let mut can_move: bool = true;
    for i in 0..n {
        if can_move {
            next_t = txy[i][0].clone();
            next_x = txy[i][1].clone();
            next_y = txy[i][2].clone();

            let distance: i64 = (now_x - next_x).abs() + (now_y - next_y).abs();
            let time_limit: i64 = next_t - now_t;

            if distance > time_limit {
                can_move = false;
                break;
            }

            if (time_limit - distance) % 2 != 0 {
                can_move = false;
                break;
            }

            now_t = next_t;
            now_x = next_x;
            now_y = next_y;
        }
    }

    if can_move {
        println!("Yes");
    } else {
        println!("No");
    }
}
