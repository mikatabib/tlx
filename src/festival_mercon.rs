use std::io;
use std::str::FromStr;

fn main() {
    let mut _n = String::new();
    io::stdin().read_line(&mut _n)
    .expect("input failed");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
    .expect("input failed");

    let count =  buffer.split_whitespace()
    .filter_map(|i| u32::from_str(i).ok())
    .filter(|i| i & 1 == 1)
    .count();

    // consume sequence of power
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
    .expect("input failed");

    println!("{:?}", (count & 1));
}
