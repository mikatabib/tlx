use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap();

    let values: Vec<String> = buffer.trim()
                                 .split_whitespace()
                                 .map(|i| i.to_string())
                                 .collect();

    let op = values[1].chars().next().unwrap();
    let a: i32 = values[0].parse().unwrap();
    let b: i32 = values[2].parse().unwrap();

    match op {
        '-' => println!("{}", a-b),
        '+' => println!("{}", a+b),
        '*' => println!("{}", a*b),
        '=' => if a==b {println!("benar");} else {println!("salah");},
        '<' => if a<b {println!("benar");} else {println!("salah");},
        '>' => if a>b {println!("benar");} else {println!("salah");},
        _ => println!(""),
    }
}
