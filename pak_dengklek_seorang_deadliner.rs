use std::io;
use std::str::FromStr;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("input failed");

    let time = (i32::from_str(&inp.trim())).unwrap();
    println!("{}", time / 3600);
    println!("{}", time / 60 % 60);
    println!("{}", time % 60);
}
