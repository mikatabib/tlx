use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .unwrap();

    let val: Vec<i32> = buffer.split_whitespace()
                       .map(|i| i.parse().unwrap())
                       .collect();
    let (a, b, c): (i32, i32, i32) = (val[0], val[1], val[2]);
    let val: i32 = 4 * val.iter().sum::<i32>() / 7;
    if a == val || b == val || c == val {println!("YA");}
    else {println!("TIDAK");}
}


