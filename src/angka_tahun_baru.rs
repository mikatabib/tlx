use std::io;

fn main() {
    let mut buff = String::new();
    io::stdin().read_line(&mut buff).unwrap();

    let n: u8 = buff.trim().parse().unwrap();
    if n == 2 || n == 3 || n == 5 {
        println!("YES");
    } else {
        println!("NO");
    }
}
