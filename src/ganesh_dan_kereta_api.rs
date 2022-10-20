use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .unwrap();

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .unwrap();

    let nums: Vec<String> = buf.trim()
                               .split_whitespace()
                               .map(|i| i.to_string())
                               .rev()
                               .collect();

    println!("{}", nums.join(","));
}
