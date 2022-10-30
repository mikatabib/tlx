use std::io;

fn main() {
    let mut buff = String::new();
    io::stdin()
        .read_line(&mut buff)
        .unwrap();

    let (_, d) = buff.trim()
                     .split_once(' ')
                     .unwrap();

    let d: u8 = d.parse().unwrap();
    if d > 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}
