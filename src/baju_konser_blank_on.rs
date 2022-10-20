use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap();

    let nums: Vec<u8> =  buffer.trim()
                                .split_whitespace()
                                .map(|i| i.parse().unwrap())
                                .collect();

    let (b, p, l) = (nums[0], nums[1], nums[2]);

    if b <= 10 && p <= 40 && l <= 90 {
        println!("S");
    } else if b <= 14 && p <= 60 && l <= 120 {
        println!("M");
    } else if b <= 18 && p <= 80 && l <= 180 {
        println!("L");
    } else {
        println!("X");
    }
}
