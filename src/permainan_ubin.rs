use std::io;

fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    let (n, m) = buff.trim()
                     .split_once(' ')
                     .unwrap();

    let n: u32 = n.parse().unwrap();
    let m: u32 = m.parse().unwrap();

    let tile = (n / 2) * (m / 2);

    println!("{}", tile * 4);
}
