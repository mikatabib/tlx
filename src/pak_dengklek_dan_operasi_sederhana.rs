use std::io;
use std::str::FromStr;

fn main() {
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("failed to read input");

    let nums: Vec<i32> = inp.split_whitespace()
                            .filter_map(|i| i32::from_str(i).ok())
                            .collect();
    let (x, y) = (nums[0], nums[1]);

    println!("{}", x + y);
    println!("{}", x - y);
    println!("{}", x * y);
    println!("{}", x / y);
    println!("{}", x % y);
}
