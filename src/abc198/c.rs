use proconio::input;

fn main() {
    input! {
        r:f64,x:f64,y:f64
    }
    let euclid_distance: f64 = (x * x + y * y).sqrt();

    if euclid_distance == (r * r).sqrt() {
        println!("1");
    } else if euclid_distance <= 2_f64 * ((r * r).sqrt()) {
        println!("2");
    } else {
        println!("{}", (euclid_distance / (r * r).sqrt()).ceil() as usize);
    }
}
