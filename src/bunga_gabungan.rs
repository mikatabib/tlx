use std::io;
use std::str::FromStr;

fn main() {
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .unwrap();

    let (p, q) = buf.trim().split_once(' ').unwrap();
    let p: u32 = u32::from_str(p).unwrap();
    let q: u32 = u32::from_str(q).unwrap();

    let n = 1+p*p+q*q;
    if n%4 != 0 {
        println!("-1");
    } else {
        println!("{}", n/4);
    }
}
