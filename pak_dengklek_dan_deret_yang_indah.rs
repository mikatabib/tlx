use std::io;
use std::str::FromStr;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("input failed");

    let nums: Vec<i32> = inp.trim()
                            .split_whitespace()
                            .filter_map(|i| i32::from_str(i).ok())
                            .collect();

    let (mut s, n, d) = (nums[0], nums[1], nums[2]);
    for _ in 0..n {
        println!("{s}");
        s += d;
    }
}
