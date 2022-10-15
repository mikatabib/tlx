use std::io;
use std::str::FromStr;

fn main() {
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read input");

    let n = i32::from_str(&inp.trim()).unwrap();
    if n < 0 || n > 100 {
        println!("TIDAK");
    } else {
        println!("YA");
    }
}
