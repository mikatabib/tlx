use std::io;
use std::str::FromStr;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read input");

    let pattern = buffer.trim();
    let (left, right) = pattern.split_once('*').unwrap();

    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read input");

    let n = i32::from_str(&buffer.trim()).unwrap();
    for _ in 0..n {
        let mut buffer = String::new();

        io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read input");

        buffer = buffer.trim().to_string();

        // skip over word that shorter than pattern
        if buffer.chars().count() < pattern.chars().count()-1 {continue;}

        // allow wildcard only and check for pattern
        if buffer.chars().count() == 1 || (buffer.starts_with(left) && buffer.ends_with(right)) {
            println!("{}", buffer);
        }
    }
}