use std::io;
fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap();

    let n: u32 = buffer.trim().parse().unwrap();
    let res = n * (n + 1) * (n * 2 + 1) / 6;
    println!("{}", res);
}
