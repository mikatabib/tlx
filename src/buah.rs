use std::io;
use std::str::FromStr;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("input failed");

    let nums: Vec<u64> = buffer.split_whitespace()
                                .filter_map(|i| u64::from_str(i).ok())
                                .collect();

    let (a, b) = (nums[1], nums[2]);
    let lcm = a * b / gcd(a, b);
    println!("{}", lcm/a + lcm/b);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            m = m ^ n;
            n = n ^ m;
            m = m ^ n;
        }
        m = m % n;
    }
    n
}