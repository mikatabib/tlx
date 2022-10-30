use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let (a, b) = buffer.trim().split_once(' ').unwrap();
    let a = a.parse().unwrap(); 
    let b = b.parse().unwrap();

    let mut total = 0;
    for i in parse(a) {
        for j in  parse(b) {
            total += i * j;
        }
    }
    println!("{}", total);
}

fn parse(mut n: i32) -> Vec<i32>{
    let digit = 1+(n as f32).log10() as usize;
    let mut nums = Vec::with_capacity(digit);
    for _ in 0..digit {
        nums.push(n%10);
        n/=10;
    }
    nums
}


